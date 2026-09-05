ENTRY(_start)

/*
 * Un seul segment est nécessaire : le programme est statique, sans sections
 * d'initialisation ni données inscriptibles.  Les sections ELF seront retirées
 * ensuite par sstrip ; seuls ces program headers servent au chargeur Linux.
 */
PHDRS
{
    load PT_LOAD FILEHDR PHDRS FLAGS(7);
}

SECTIONS
{
    . = 0x200000 + SIZEOF_HEADERS;

    .rodata : ALIGN(1)
    {
        *(.rodata .rodata.*)
    } :load

    .text : ALIGN(1)
    {
        *(.text .text.*)
    } :load

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.comment)
        *(.note .note.*)
        *(.debug*)
    }
}
