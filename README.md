# MyInit TODO List

This document outlines the planned features and learning milestones for the MyInit project - a minimal init system for learning how Linux initialization works.

### WARNING
Please dont use this init system for production environments. It is intended for educational purposes only.

## Requirements
- qemu-system-x86_64
- busybox
- rustup
- cpio

## Current Status
- [x] Basic init system that can mount essential filesystems
- [x] Process management with fork/exec and reaping
- [x] Simple service management
- [x] Simple Command Line Interface
- [x] Add unix socket for messages between services and command line utilities
- [x] Implement shutdown sequence

## Goals

### Service Enhancements
- [ ] Implement service dependency resolution
- [ ] Add service state tracking (starting, running, stopped, failed)
- [ ] Implement signal handling (SIGTERM, SIGINT)
- [ ] Add automatic service restart capability
- [ ] Track and log service exit codes
- [ ] Add timeout handling for service startup
- [ ] Create a simple configuration file format
- [ ] Add logging capabilities
- [ ] Add signal handling for shutdown/reboot

### Core Utilities (Not really related to the init system just for fun i guess)
- [ ] Replace BusyBox with custom implementations
- [ ] Implement minimal versions of essential utilities:
  - [ ] `ls` - List directory contents
  - [ ] `cat` - Concatenate files
  - [ ] `echo` - Display text
  - [ ] `ps` - Show process status
  - [ ] `kill` - Send signals to processes
  - [ ] `myshell` - A minimal shell

### Learning Milestones
- [ ] Understand process lifecycle in detail
- [ ] Learn how to properly manage process groups and sessions
- [ ] Add socket activation for services
