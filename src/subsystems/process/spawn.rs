use std::ffi::{CStr, CString};
use std::path::Path;

pub fn spawn_process(cmd: &str) -> Result<u32, crate::Error> {
    let args: Vec<&str> = cmd.split_whitespace().collect();
    if args.is_empty() {
        return Err(crate::Error {
            message: "Empty command".into(),
            fatal: false,
        });
    }

    unsafe {
        let child = nix::unistd::fork()?;

        match child {
            nix::unistd::ForkResult::Parent { child } => Ok(child.as_raw() as u32),
            nix::unistd::ForkResult::Child => {
                // Resolve the binary path - if it doesn't start with '/', look in /bin
                let binary_path = if args[0].starts_with('/') {
                    args[0].to_string()
                } else {
                    format!("/bin/{}", args[0])
                };

                // Check if the file exists before trying to execute it
                if !Path::new(&binary_path).exists() {
                    eprintln!("Binary not found: {}", binary_path);
                    std::process::exit(127); // Standard "command not found" exit code
                }

                // Create a CString for the executable path
                let filename = CString::new(binary_path.clone()).expect("Invalid filename");

                // Convert arguments to CStrings
                let c_args: Vec<CString> = std::iter::once(CString::new(binary_path).unwrap())
                    .chain(args.iter().skip(1).map(|arg| CString::new(*arg).unwrap()))
                    .collect();

                // Convert to a vector of references that execv expects
                let c_args_refs: Vec<&CStr> = c_args.iter().map(|s| s.as_c_str()).collect();

                eprintln!(
                    "Executing: {} with args: {:?}",
                    filename.to_string_lossy(),
                    args
                );

                // Use execv instead of execvp to specify the exact path
                match nix::unistd::execv(&filename, &c_args_refs) {
                    Ok(_) => unreachable!(),
                    Err(e) => {
                        eprintln!("Execution failed: {:?}", e);
                        std::process::exit(126); // Standard "execution failed" exit code
                    }
                }
            }
        }
    }
}
