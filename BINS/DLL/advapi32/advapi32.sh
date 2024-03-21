objdump -D  ./advapi32.dll  > advapi32.objdump.att.asm
objdump -x  ./advapi32.dll  > advapi32.objdump.header
objdump -f  ./advapi32.dll  > advapi32.objdump.fileheader
objdump -p  ./advapi32.dll  > advapi32.objdump.privatefileheader
objdump -t  ./advapi32.dll  > advapi32.objdump.symboltable
objdump -T  ./advapi32.dll  > advapi32.objdump.dynamicsymboltable
objdump -R  ./advapi32.dll  > advapi32.objdump.dynamicrelocationtable
objdump -r  ./advapi32.dll  > advapi32.objdump.relocationtable
