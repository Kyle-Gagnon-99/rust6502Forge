// Linker Script for NES

// Define the segments where the data/code will reside.
SEGMENTS {
    ZEROPAGE: load = ZP, type = zp;
    BSS:      load = RAM, type = bss;
    INES:     load = HEADER, type = ro, align = $10;
    CODE:     load = PRG0, type = ro;
    VECTOR:   load = PRG0, type = ro, start = $BFFA;
    CHR0a:    load = CHR0a, type = ro;
    CHR0b:    load = CHR0b, type = ro;
}

// Define the memory areas, their start addresses, sizes, and properties.
MEMORY {
    ZP:       start = $0000, size = $0100, type = rw;
    RAM:      start = $0300, size = $0400, type = rw;
    HEADER:   start = $0000, size = $0010, type = rw, fill = yes;
    PRG0:     start = $8000, size = $4000, type = ro, fill = yes;
    CHR0a:    start = $0000, size = $1000, type = ro, fill = yes;
    CHR0b:    start = $1000, size = $1000, type = ro, fill = yes;
}