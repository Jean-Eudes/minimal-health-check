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

    .text : SUBALIGN(1)
    {
        *(.rodata .rodata.*)
        *(.text .text.*)
    } :load

    .data : SUBALIGN(1)
    {
        *(.data .data.*)
        *(.sdata .sdata.*)
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
