# Security

This tool reads your Plasma config files but doesn't do anything malicious with them.

## What it does
- Reads your local config files
- Creates restoration scripts 
- Optionally copies config files to bundles

## What it doesn't do  
- Modify system files
- Access files outside your home directory
- Send data anywhere
- Run untrusted code

## Bundle safety
- Exported bundles contain your desktop preferences
- Review `layout.json` before sharing bundles
- Restoration scripts run with your user privileges

## Report issues
Found a security problem? Email: security@example.com

Pretty straightforward - it's just a backup tool for your desktop setup.