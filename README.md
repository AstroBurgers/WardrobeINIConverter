# WardrobeINIConverter
⚠️ Disclaimer:
I am not responsible for any issues or damage to your game files. Use at your own risk.

## Overview

**WardrobeINIConverter** is a standalone tool (CLI executable) designed to convert the contents of your `wardrobe.ini` from EUP format into UltimateBackup-compatible Ped Entries.

## Features
- Efficiently parses large wardrobe.ini files.
- Converts EUP wardrobe data into the UltimateBackup XML Ped Entry format.
- Supports multi-threaded parsing for performance.

## Performance

Tested on a synthetic `wardrobe.ini` with **1 million entries** on a Ryzen 5600 6-core OC @ 4.4 GHz:

| Run        | Time (ms)  |
| ---------- | ---------- |
| 1          | 3618       |
| 2          | 3875       |
| 3          | 3666       |
| 4          | 3904       |
| 5          | 3586       |
| 6          | 3588       |
| 7          | 3578       |
| 8          | 3786       |
| 9          | 3839       |
| 10         | 3762       |
| **Median** | **3714**   |
| **Mean**   | **3720.2** |
---
*What does this mean in layman's terms?*</br>
**On a regular Wardrobe.ini file it runs *literally* faster than you can blink (in testing, 3.6ms)**

# Installation & Usage
⚠️ Disclaimer:
If the files are NOT put in your GTA5 Root folder, the program will quit unexpectedly and not execute. **Do not ask for support for improper installations**
1. Download the rar from the releases.
2. Extract the contents of the rar somewhere safe
3. Drag and Drop the files into your GTA5 root folder (where your GTA5.exe is)
4. Run the executable
5. Fetch your freshly converted Ped Entries from `Grand Theft Auto V\WardrobeINIConverter\ConvertedLines.txt`
