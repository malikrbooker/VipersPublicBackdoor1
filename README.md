# Rust Malware Concept
A Concept design of a backdoor that utilizes HTTP/TCP sockets,  
Caesar encryption, api.smsglobal.com (for SMS), &amp; more.  

### Table of Contents
- [How it works](#how-it-works)
- [Building It Yourself](#building-it-yourself)

### How it works
#### TL;DR
The file moves itself, renames itself (as backdoor.exe), hides itself, and re-executes itself  
inside of the target's **AppData** folder.

**Detailed explanation**  
The application is a Trojan that moves itself into the users <code>AppData</code> folder and sets its  
attribute to hidden. The file then pings a user-defined host server that holds the malicious  
payload and downloads it. In my case this file was the <code>Nishang Invoke-PowershellTcp.ps1</code>  
You can set this to whatever you want obviously. Once the Payload is downloaded, the  
application then attempts to run it with a reverse shell to another user-defined listening  
server (In this case, defaulted to port 8080).  

If successful, the listening receiver will have a Powershell like environment to do  
whatever the receiver pleases with the victim's pc.

### Building It Yourself
If you compile this code as is, it IS NOT malicious in any way shape or form. In fact, the  
program will run for about 5 minutes before aborting.

Things to look into changing include:
 - User-defined hostname that hosts the payload.
 - User-defined hostname that listens for the activation signal.
 - The actual payload that is hosted itself.

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install MinGW-w64
sudo apt install mingw-w64

# Set up the Rust toolchain for x86_64 Windows target
rustup target add x86_64-pc-windows-gnu

# Build the concept binary
cargo clean
cargo build
```
