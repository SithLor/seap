objdump -D  ./ntdll.dll  > ntdll.objdump.att.asm
objdump -x  ./ntdll.dll  > ntdll.objdump.header
objdump -f  ./ntdll.dll  > ntdll.objdump.fileheader
objdump -p  ./ntdll.dll  > ntdll.objdump.privatefileheader
objdump -t  ./ntdll.dll  > ntdll.objdump.symboltable
#objdump -T  ./ntdll.dll  > ntdll.objdump.dynamicsymboltable
#objdump -R  ./ntdll.dll  > ntdll.objdump.dynamicrelocationtable
objdump -r  ./ntdll.dll  > ntdll.objdump.relocationtable
