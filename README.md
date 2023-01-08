# outlook_autocomplete_reader

A simple rust tool that can read an outlook contacts autocomplete file and converts it to a csv or vcf file.

## Download and compiling

Precompiled builds can be downloaded from my [jenkins server](https://ci.howaner.de/job/outlook_autocomplete_reader/).
Alternatively, the program can be compiled with the following command: ```cargo build --release```

## Usage

1. Close and reopen outlook to ensure the cached autocomplete file is up-to-date.
2. Find the cached outlook autocomplete file. It should be in **%LOCALAPPDATA%\Microsoft\Outlook\RoamCache** and begins with Stream_Autocomplete_...
3. If the file does not exist, it is possible to extract this file from your outlook data file. How to do this can be read at [Microsoft](https://support.microsoft.com/en-us/office/import-or-copy-the-autocomplete-list-to-another-computer-83558574-20dc-4c94-a531-25a42ec8e8f0)
4. Download the outlook_autocomplete_reader tool from my [jenkins server](https://ci.howaner.de/job/outlook_autocomplete_reader/) and execute it with the command line:
```
# Converts the autocomplete list to a csv
outlook_autocomplete_reader-win64.exe --file C:\Users\xxx\AppData\Local\Microsoft\Outlook\RoamCache\Stream_Autocomplete_0_xxxx.dat -o contacts.csv --output-format csv
# Converts the autocomplete list to a vcard file
outlook_autocomplete_reader-win64.exe --file C:\Users\xxx\AppData\Local\Microsoft\Outlook\RoamCache\Stream_Autocomplete_0_xxxx.dat -o contacts.vcf --output-format vcard
```
5. If something is not working, please execute the tool with the verbose flag (-v) and create a github issue.
