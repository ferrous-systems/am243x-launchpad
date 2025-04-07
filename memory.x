/*
Memory configuration for the Texas Instruments AM243x Launchpad.
*/

MEMORY {
    MSRAM : ORIGIN = 0x70000000, LENGTH = 2M
}

REGION_ALIAS("CODE", MSRAM);
REGION_ALIAS("DATA", MSRAM);
