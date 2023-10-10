# get_csgo_movement_in_cs2

An easier way to add some lines to the autoexec for people prefering the easy way.

---

### **Notice**

**Windows defender currently detects this as malware.**

**As of now Unix platforms like Linux and Mac are not supported.**

Feel free to audit and contribute.

---

### What does it do?

This program adds some binds to the autoexec file.
If it does not exist it creates a new one.

The commands are found in this tweet: https://twitter.com/SPUNJ/status/1709625427625447781

Note this can be done manualy, this program is here to make this more accessible for less tech savy people.

Technical explanation down near the end of the readme.

### How to use

1. Add Launch Options
   
   ![Open Properties](/images/ClickProperties.png)
   ![Launch options](/images/LaunchOptions.png)

2. Run get_csgo_movement_in_cs2.exe

3. Done!

---

### Running into issues?

If you are running into issues you can either

1. Contact me on Discord at .sakuraphoenix

2. Create a issue on Github, make sure to include information like
   
   - Os aka are you running Windows, Linux or Mac plus version.
   
   - The error you got.
   
   - And any other potentialy relevent info.

---

### Code of conduct

Nothing really, Just follow Kardemome loven

"You shouldn't bother others, you should be decent and kind, and otherwise you can do what you want."

---


##### Technical explanation:

1. First it finds cs2 by reading registry key
   
   "HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Valve\cs2" on 64bit and "HKEY_LOCAL_MACHINE\Valve\cs2" on 32bit. variable "installpath"
   
   ```rust
   // src/cs2_locator.rs
   fn get_registry_path() -> Result<&'s str, GetRegistryPathError> {
      match bitness::os_bitness()? {
          Bitness::X86_32 => Ok("SOFTWARE\\Valve\\cs2"),
          Bitness::X86_64 => Ok("SOFTWARE\\WOW6432Node\\Valve\\cs2"),
          Bitness::Unknown => Err(GetRegistryPathError::Unknown),
      }
   }
   ```

2. Append the relative path to the autoexec file
   
   ```rust
   // src/cs2_locator.rs
   pub fn get_autoexec_path(game_path: String) -> String {
      format!("{}\\game\\csgo\\cfg\\autoexec.cfg", game_path)
   }
   ```

  3. Check if the commands are already there. If it is then exit, else append the commands to the file, and exit.

Here are the relevant functions:

```rust
// src/auto_exec
pub fn is_duplicate(&self) -> Result<bool, AutoExecError> {
        if !Path::new(&self.path).exists() {
            return Ok(false);
        }

        let content = fs::read_to_string(&self.path)?;
        if content.trim().contains(self.preset.trim()) {
            return Ok(true)
        }

        Ok(false)
    }

pub fn append(&self) -> Result<(), AutoExecError> {
    if !Path::new(&self.path).exists() {
        File::create(&self.path)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&self.path)?;

    writeln!(file, "{}", &self.preset)?;
    Ok(())
}
```
