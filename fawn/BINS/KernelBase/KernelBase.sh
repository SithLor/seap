objdump -D  ./KernelBase.dll  > KernelBase.objdump.att.asm
objdump -x  ./KernelBase.dll  > KernelBase.objdump.header
objdump -f  ./KernelBase.dll  > KernelBase.objdump.fileheader
objdump -p  ./KernelBase.dll  > KernelBase.objdump.privatefileheader
objdump -t  ./KernelBase.dll  > KernelBase.objdump.symboltable
objdump -T  ./KernelBase.dll  > KernelBase.objdump.dynamicsymboltable
objdump -R  ./KernelBase.dll  > KernelBase.objdump.dynamicrelocationtable
objdump -r  ./KernelBase.dll  > KernelBase.objdump.relocationtable
