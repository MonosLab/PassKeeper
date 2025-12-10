# PassKeeper Tutorial

This tutorial will walk you through using PassKeeper, a secure password management application.

## Getting Started

### 1. First Launch

When you first launch PassKeeper, you'll see the unlock screen:

![Unlock Screen]
- Enter a master password that you'll use to access your passwords
- **IMPORTANT**: Remember this password! It cannot be recovered
- Choose a strong password (12+ characters recommended)

### 2. Main Interface

After unlocking, you'll see three main sections:
- **Search Bar**: Find passwords quickly by title, username, or URL
- **Add Password Button**: Create a new password entry
- **Lock Button**: Lock the application when done

## Adding Your First Password

1. Click the **"+ Add Password"** button
2. Fill in the required fields:
   - **Title**: Name for this password (e.g., "Gmail", "Facebook")
   - **Username/Email**: Your login username or email
   - **Password**: Your password for this service
3. Optional fields:
   - **URL**: Website address
   - **Notes**: Any additional information
4. Click **Save**

## Using the Password Generator

Instead of creating your own password:

1. Click **"+ Add Password"** or edit an existing password
2. Click the **"Generate"** button next to the password field
3. Customize your password:
   - Adjust the **length** slider (8-128 characters)
   - Check/uncheck options:
     - Uppercase letters (A-Z)
     - Numbers (0-9)
     - Symbols (!@#$...)
4. Click **"Regenerate"** to try different combinations
5. Click **"Use This Password"** to apply it to your entry

## Managing Passwords

### Viewing a Password
1. Find the password card in your list
2. Click the **"View"** button
3. The password is automatically copied to your clipboard
4. Paste it where needed

### Editing a Password
1. Click the **"Edit"** button on any password card
2. Update the fields you want to change
3. Click **"Save"**

### Deleting a Password
1. Click the **"Delete"** button on any password card
2. Confirm the deletion
3. The password is permanently removed

### Searching for Passwords
1. Type in the search bar at the top
2. Results are filtered in real-time
3. Search matches title, username, and URL fields

## Security Best Practices

### DO:
✅ Use a strong, unique master password  
✅ Lock the application when not in use  
✅ Generate strong passwords for each service  
✅ Keep the application updated  
✅ Back up your encrypted password file regularly  

### DON'T:
❌ Share your master password with anyone  
❌ Use the same password for multiple services  
❌ Leave the application unlocked on shared computers  
❌ Store your master password anywhere  
❌ Use weak or common passwords  

## Keyboard Shortcuts

While the application is open:
- **Search**: Click on the search bar or start typing
- **Lock**: Click the lock button when done
- **Escape**: Close modals

## Tips and Tricks

### Organizing Passwords
- Use clear, descriptive titles (e.g., "Work Email" vs "Email 1")
- Include account identifiers in notes for multiple accounts
- Use the URL field to quickly identify services

### Password Strength
- Minimum 12 characters for important accounts
- Mix uppercase, lowercase, numbers, and symbols
- Avoid dictionary words or personal information
- Use different passwords for each service

### Regular Maintenance
- Review your passwords monthly
- Update weak passwords
- Remove passwords for accounts you no longer use
- Update compromised passwords immediately

## Common Questions

**Q: What if I forget my master password?**  
A: Unfortunately, there is no way to recover a forgotten master password. This is by design for security. Always remember your master password or keep a secure backup.

**Q: Where are my passwords stored?**  
A: Passwords are stored encrypted on your local device in:
- Linux: `~/.local/share/passkeeper/`
- macOS: `~/Library/Application Support/passkeeper/`
- Windows: `%APPDATA%\passkeeper\`

**Q: Can I use this on multiple devices?**  
A: Currently, PassKeeper stores data locally. To use on multiple devices, you would need to manually sync the encrypted password file.

**Q: Is this secure?**  
A: PassKeeper uses AES-256-GCM encryption for storing passwords. However, see SECURITY.md for important information about key derivation and production use.

**Q: Can I export my passwords?**  
A: Currently, there is no export feature. This is a potential future enhancement.

**Q: What happens if I lose the password file?**  
A: Without a backup, your passwords would be lost. Make regular backups of your encrypted password file.

## Troubleshooting

### Application won't unlock
- Double-check your master password
- Ensure caps lock is off
- Try restarting the application

### Can't see my passwords
- Make sure the application is unlocked
- Try refreshing by locking and unlocking
- Check if the search bar has text (clear it)

### Password not copied to clipboard
- Try clicking "View" again
- Check system clipboard permissions
- Some systems require clipboard access permission

## Need Help?

If you encounter issues or have questions:
1. Check this tutorial
2. Review the README.md file
3. Check the SECURITY.md for security-related questions
4. Open an issue on the GitHub repository

---

Happy password managing! Stay secure! 🔐
