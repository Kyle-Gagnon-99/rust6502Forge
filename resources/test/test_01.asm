.include "constants.asm"

;;; Size of PRG in units of 16 KiB.
prg_npage = 1
;;; Size of CHR in units of 8 KiB.
chr_npage = 1
;;; INES mapper number.
mapper = 0
;;; Mirroring (0 = horizontal, 1 = vertical)
mirroring = 1

;;; INES Header
.segment "INES"
.byte $4e, $45, $53, $1a
.byte prg_npage
.byte chr_npage
.byte ((mapper & $0f) << 4) | (mirroring & 1)
.byte mapper & $f0

;;; Reset Vector
.segment "VECTOR"
.addr nmi
.addr reset
.addr irq

;;; Actual code
.code

;;; Player enum
.enum Direction
    NORTH = 0
    SOUTH = 1
    EAST = 2
    WEST = 3
.endenum

;;; Non-maskable interrupt
.proc nmi
    rti
.endproc

;;; IRQ
.proc irq
    rti
.endproc


.proc reset         ; Ending
    LDA #$44            ; The comment is right next to the operand
    STA PPUSTATUS + 1
.endproc

.scope Player
    .proc main
        constant = $1200
        @loop:
            lda #constant
            sta #Direction::NORTH
            ldy (Direction::SOUTH,X)
            ldx Direction::EAST,X
            jmp @loop
    .endproc
.endscope