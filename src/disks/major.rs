pub enum Major {
    Unnamed, // 0        Unnamed devices (e.g. non-device mounts)
    RamDisk, //   1 block    RAM disk
    FloppyDisk, //   2 block    Floppy disks
    Ide, //   3 block    First MFM, RLL and IDE hard disk/CD-ROM interface
    Dynamic, //   4 block    Aliases for dynamically allocated major devices to be used
    Loopback, //   7 block    Loopback devices
    ScsiDisk, //   8 block    SCSI disk devices (0-15)
    RaidMetaDisk, //   9 block    Metadisk (RAID) devices
    ScsiCdRom, //  11 block    SCSI CD-ROM devices
    //  12 block
    XTDisk, //  13 block    Previously used for the XT disk (/dev/xdN)
    //  14 block
    SonyCdRom, //  15 block    Sony CDU-31A/CDU-33A CD-ROM
    GoldStarCdRom, //  16 block    GoldStar CD-ROM
    OpticsStorageCdRom, //  17 block    Optics Storage CD-ROM
    SanyoCdRom, //  18 block    Sanyo CD-ROM
    DoubleCompressedDisk, //  19 block    "Double" compressed disk
    HitachiCdRom, //  20 block    Hitachi CD-ROM (under development)
    AcornMfmDrive, //  21 block    Acorn MFM hard drive interface
    SecondsIde, //  22 block    Second IDE hard disk/CD-ROM interface
    //  23 block    Mitsumi proprietary CD-ROM
    //  24 block    Sony CDU-535 CD-ROM
    //  25 block    First Matsushita (Panasonic/SoundBlaster) CD-ROM
    //  26 block    Second Matsushita (Panasonic/SoundBlaster) CD-ROM
    //  27 block    Third Matsushita (Panasonic/SoundBlaster) CD-ROM
    //  28 block    Fourth Matsushita (Panasonic/SoundBlaster) CD-ROM
    //  28 block    ACSI disk (68k/Atari)
    //  29 block    Aztech/Orchid/Okano/Wearnes CD-ROM
    //  30 block    Philips LMS CM-205 CD-ROM
    //  31 block    ROM/flash memory card
    //  32 block    Philips LMS CM-206 CD-ROM
    //  33 block    Third IDE hard disk/CD-ROM interface
    //  34 block    Fourth IDE hard disk/CD-ROM interface
    //  35 block    Slow memory ramdisk
    //  36 block    OBSOLETE (was MCA ESDI hard disk)
    //  37 block    Zorro II ramdisk
    //  38 block    OBSOLETE (was Linux/AP+)
    //  39 block
    //  40 block
    //  41 block
    //  42 block    Demo/sample use
    //  43 block    Network block devices
    //  44 block    Flash Translation Layer (FTL) filesystems
    //  45 block    Parallel port IDE disk devices
    //  46 block    Parallel port ATAPI CD-ROM devices
    //  47 block    Parallel port ATAPI disk devices
    //  48 block    Mylex DAC960 PCI RAID controller; first controller
    //  49 block    Mylex DAC960 PCI RAID controller; second controller
    //  50 block    Mylex DAC960 PCI RAID controller; third controller
    //  51 block    Mylex DAC960 PCI RAID controller; fourth controller
    //  52 block    Mylex DAC960 PCI RAID controller; fifth controller
    //  53 block    Mylex DAC960 PCI RAID controller; sixth controller
    //  54 block    Mylex DAC960 PCI RAID controller; seventh controller
    //  55 block    Mylex DAC960 PCI RAID controller; eighth controller
    //  56 block    Fifth IDE hard disk/CD-ROM interface
    //  57 block    Sixth IDE hard disk/CD-ROM interface
    //  58 block    Reserved for logical volume manager
    //  59 block    Generic PDA filesystem device
    //  60-63 block    LOCAL/EXPERIMENTAL USE
    //  64 block    Scramdisk/DriveCrypt encrypted devices
    //  65 block    SCSI disk devices (16-31)
    //  66 block    SCSI disk devices (32-47)
    //  67 block    SCSI disk devices (48-63)
    //  68 block    SCSI disk devices (64-79)
    //  69 block    SCSI disk devices (80-95)
    //  70 block    SCSI disk devices (96-111)
    //  71 block    SCSI disk devices (112-127)
    //  72 block    Compaq Intelligent Drive Array, first controller
    //  73 block    Compaq Intelligent Drive Array, second controller
    //  74 block    Compaq Intelligent Drive Array, third controller
    //  75 block    Compaq Intelligent Drive Array, fourth controller
    //  76 block    Compaq Intelligent Drive Array, fifth controller
    //  77 block    Compaq Intelligent Drive Array, sixth controller
    //  78 block    Compaq Intelligent Drive Array, seventh controller
    //  79 block    Compaq Intelligent Drive Array, eighth controller
    //  80 block    I2O hard disk
    //  81 block    I2O hard disk
    //  82 block    I2O hard disk
    //  83 block    I2O hard disk
    //  84 block    I2O hard disk
    //  85 block    I2O hard disk
    //  86 block    I2O hard disk
    //  87 block    I2O hard disk
    //  88 block    Seventh IDE hard disk/CD-ROM interface
    //  89 block    Eighth IDE hard disk/CD-ROM interface
    //  90 block    Ninth IDE hard disk/CD-ROM interface
    //  91 block    Tenth IDE hard disk/CD-ROM interface
    //  92 block    PPDD encrypted disk driver
    //  93 block    NAND Flash Translation Layer filesystem
    //  94 block    IBM S/390 DASD block storage
    //  96 block    Inverse NAND Flash Translation Layer
    //  98 block    User-mode virtual block device
    //  99 block    JavaStation flash disk
    // 101 block    AMI HyperDisk RAID controller
    // 102 block    Compressed block device
    // 103 block    Audit device
    // 104 block    Compaq Next Generation Drive Array, first controller
    // 105 block    Compaq Next Generation Drive Array, second controller
    // 106 block    Compaq Next Generation Drive Array, third controller
    // 107 block    Compaq Next Generation Drive Array, fourth controller
    // 108 block    Compaq Next Generation Drive Array, fifth controller
    // 109 block    Compaq Next Generation Drive Array, sixth controller
    // 110 block    Compaq Next Generation Drive Array, seventh controller
    // 111 block    Compaq Next Generation Drive Array, eighth controller
    // 112 block    IBM iSeries virtual disk
    // 113 block    IBM iSeries virtual CD-ROM
    // 114 block       IDE BIOS powered software RAID interfaces such as the
    // 115 block       NetWare (NWFS) Devices (0-255)
    // 116 block       MicroMemory battery backed RAM adapter (NVRAM)
    // 117 block       Enterprise Volume Management System (EVMS)
    // 120-127 block    LOCAL/EXPERIMENTAL USE
    // 128 block       SCSI disk devices (128-143)
    // 129 block       SCSI disk devices (144-159)
    // 130 block       SCSI disk devices (160-175)
    // 131 block       SCSI disk devices (176-191)
    // 132 block       SCSI disk devices (192-207)
    // 133 block       SCSI disk devices (208-223)
    // 134 block       SCSI disk devices (224-239)
    // 135 block       SCSI disk devices (240-255)
    // 136 block    Mylex DAC960 PCI RAID controller; ninth controller
    // 137 block    Mylex DAC960 PCI RAID controller; tenth controller
    // 138 block    Mylex DAC960 PCI RAID controller; eleventh controller
    // 139 block    Mylex DAC960 PCI RAID controller; twelfth controller
    // 140 block    Mylex DAC960 PCI RAID controller; thirteenth controller
    // 141 block    Mylex DAC960 PCI RAID controller; fourteenth controller
    // 142 block    Mylex DAC960 PCI RAID controller; fifteenth controller
    // 143 block    Mylex DAC960 PCI RAID controller; sixteenth controller
    // 144 block    Expansion Area #1 for more non-device (e.g. NFS) mounts
    // 145 block    Expansion Area #2 for more non-device (e.g. NFS) mounts
    // 146 block    Expansion Area #3 for more non-device (e.g. NFS) mounts
    // 147 block    Distributed Replicated Block Device (DRBD)
    // 152 block    EtherDrive Block Devices
    // 153 block    Enhanced Metadisk RAID (EMD) storage units
    // 159 block    RESERVED
    // 160 block       Carmel 8-port SATA Disks on First Controller
    // 161 block       Carmel 8-port SATA Disks on Second Controller
    // 179 block       MMC block devices
    // 180 block    USB block devices
    // 199 block    Veritas volume manager (VxVM) volumes
    // 201 block    Veritas VxVM dynamic multipathing driver
    // 202 block    Xen Virtual Block Device
    // 240-254 block    LOCAL/EXPERIMENTAL USE
    // 255 block    RESERVED
    // 256 block    Resident Flash Disk Flash Translation Layer
    // 257 block    SSFDC Flash Translation Layer filesystem
    // 258 block    ROM/Flash read-only translation layer
    // 259 block    Block Extended Major
    NotSupported,
}

impl From<u32> for Major {
    fn from(major: u32) -> Self {
        match major {
            0 => Major::Unnamed,
            1 => Major::RamDisk,
            2 => Major::FloppyDisk,
            3 => Major::Ide,
            4 => Major::Dynamic,
            7 => Major::Loopback,
            8 => Major::ScsiDisk,
            9 => Major::RaidMetaDisk,
            11 => Major::ScsiCdRom,
            13 => Major::XTDisk,
            15 => Major::SonyCdRom,
            16 => Major::GoldStarCdRom,
            17 => Major::OpticsStorageCdRom,
            18 => Major::SanyoCdRom,
            19 => Major::DoubleCompressedDisk,
            20 => Major::HitachiCdRom,
            21 => Major::AcornMfmDrive,
            22 => Major::SecondsIde,
            _ => Major::NotSupported,
        }
    }
}
