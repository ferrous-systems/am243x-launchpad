/*
Memory configuration for the Texas Instruments AM243x Launchpad.
*/

MEMORY {
    /* This is at the start of TCMA */
    R5F_VECTORS (RWX) : ORIGIN = 0x00000000, LENGTH = 0x40
    R5F_TCMA    (RWX) : ORIGIN = 0x00000040, LENGTH = 0x7FC0
    R5F_TCMB0   (RWX) : ORIGIN = 0x41010000 , LENGTH = 0x8000 
    /* The bootloader uses 0x7018_0000 to 0x701F_FFFF */
    MSRAM       (RWX) : ORIGIN = 0x70000000, LENGTH = 0x100000
    MSRAM_NC    (RWX) : ORIGIN = 0x70100000, LENGTH = 0x80000
    FLASH        (RX) : ORIGIN = 0x60100000, LENGTH = 0x80000
}

REGION_ALIAS("VECTORS", R5F_VECTORS);
REGION_ALIAS("CODE", MSRAM);
REGION_ALIAS("DATA", MSRAM);
REGION_ALIAS("STACKS", MSRAM);

/* Code Composer Studio needs to see this symbol */
PROVIDE(SVC_Handler = _asm_svc_handler);
EXTERN(SVC_Handler);

PROVIDE(_hyp_stack_size = 16K);
PROVIDE(_und_stack_size = 16K);
PROVIDE(_svc_stack_size = 16K);
PROVIDE(_abt_stack_size = 16K);
PROVIDE(_irq_stack_size = 64);
PROVIDE(_fiq_stack_size = 64);
PROVIDE(_sys_stack_size = 16K);

SECTIONS {
    .core1_stacks (NOLOAD) : ALIGN(8)
    {
        . = ALIGN(8);
        _core1_stacks_low_end = .;
        _core1_sys_stack_end = .;
        . += _sys_stack_size;
        . = ALIGN(8);
        _core1_sys_stack = .;
        _core1_fiq_stack_end = .;
        . += _fiq_stack_size;
        . = ALIGN(8);
        _core1_fiq_stack = .;
        _core1_irq_stack_end = .;
        . += _irq_stack_size;
        . = ALIGN(8);
        _core1_irq_stack = .;
        _core1_abt_stack_end = .;
        . += _abt_stack_size;
        . = ALIGN(8);
        _core1_abt_stack = .;
        _core1_svc_stack_end = .;
        . += _svc_stack_size;
        . = ALIGN(8);
        _core1_svc_stack = .;
        _core1_und_stack_end = .;
        . += _und_stack_size;
        . = ALIGN(8);
        _core1_und_stack = .;
        _core1_hyp_stack_end = .;
        . += _hyp_stack_size;
        . = ALIGN(8);
        _core1_hyp_stack = .;
        _core1_stacks_high_end = .;
    } > STACKS
} INSERT BEFORE .filler;

PROVIDE(kmain2 = default_kmain2);
