//! Use the registry examples to validate the code. As you can see, there are a lot of errors/exceptions, unfortunately.

use iban::{Iban, IbanLike, ParseIbanError};

struct RegistryExample<'a> {
    country_code: &'a str,
    bank_identifier: Option<&'a str>,
    branch_identifier: Option<&'a str>,
    bban: &'a str,
    iban_electronic: &'a str,
    iban_print: &'a str,
}

#[test]
fn test_registry_examples() -> Result<(), ParseIbanError> {
    //
    let examples: &[RegistryExample] = &[
        RegistryExample {
            country_code: "AD",
            bank_identifier: Some("0001"),
            branch_identifier: Some("2030"),
            bban: "00012030200359100100",
            iban_electronic: "AD1200012030200359100100",
            iban_print: "AD12 0001 2030 2003 5910 0100",
        },
        RegistryExample {
            country_code: "AE",
            bank_identifier: Some("033"),
            branch_identifier: None,
            bban: "0331234567890123456",
            iban_electronic: "AE070331234567890123456",
            iban_print: "AE07 0331 2345 6789 0123 456",
        },
        RegistryExample {
            country_code: "AL",
            bank_identifier: Some("212-1100-9"),
            branch_identifier: Some("1100"),
            bban: "212110090000000235698741",
            iban_electronic: "AL47212110090000000235698741",
            iban_print: "AL47 2121 1009 0000 0002 3569 8741",
        },
        RegistryExample {
            country_code: "AT",
            bank_identifier: Some("19043"),
            branch_identifier: None,
            bban: "1904300234573201",
            iban_electronic: "AT611904300234573201",
            iban_print: "AT61 1904 3002 3457 3201",
        },
        RegistryExample {
            country_code: "AZ",
            bank_identifier: Some("NABZ"),
            branch_identifier: None,
            bban: "NABZ00000000137010001944",
            iban_electronic: "AZ21NABZ00000000137010001944",
            iban_print: "AZ21 NABZ 0000 0000 1370 1000 1944",
        },
        RegistryExample {
            country_code: "BA",
            bank_identifier: Some("199"),
            branch_identifier: Some("044"),
            bban: "1990440001200279",
            iban_electronic: "BA391290079401028494",
            iban_print: "BA39 1290 0794 0102 8494",
        },
        RegistryExample {
            country_code: "BE",
            bank_identifier: Some("539"),
            branch_identifier: None,
            bban: "539007547034",
            iban_electronic: "BE68539007547034",
            iban_print: "BE68 5390 0754 7034",
        },
        RegistryExample {
            country_code: "BG",
            bank_identifier: Some("BNBG"),
            branch_identifier: Some("9661"),
            bban: "BNBG96611020345678",
            iban_electronic: "BG80BNBG96611020345678",
            iban_print: "BG80 BNBG 9661 1020 3456 78",
        },
        RegistryExample {
            country_code: "BH",
            bank_identifier: Some("BMAG"),
            branch_identifier: None,
            bban: "BMAG00001299123456",
            iban_electronic: "BH67BMAG00001299123456",
            iban_print: "BH67 BMAG 0000 1299 1234 56",
        },
        RegistryExample {
            country_code: "BI",
            bank_identifier: Some("10000"),
            branch_identifier: Some("10001"),
            bban: "10000100010000332045181",
            iban_electronic: "BI4210000100010000332045181",
            iban_print: "BI42 10000 10001 00003320451 81",
        },
        RegistryExample {
            country_code: "BR",
            bank_identifier: Some("00360305"),
            branch_identifier: Some("00001"),
            bban: "00360305000010009795493P1",
            iban_electronic: "BR1800360305000010009795493C1",
            iban_print: "BR18 0036 0305 0000 1000 9795 493C 1",
        },
        RegistryExample {
            country_code: "BY",
            bank_identifier: Some("NBRB"),
            branch_identifier: None,
            bban: "NBRB 3600900000002Z00AB00",
            iban_electronic: "BY13NBRB3600900000002Z00AB00",
            iban_print: "BY13 NBRB 3600 9000 0000 2Z00 AB00",
        },
        RegistryExample {
            country_code: "CH",
            bank_identifier: Some("00762"),
            branch_identifier: None,
            bban: "00762011623852957",
            iban_electronic: "CH9300762011623852957",
            iban_print: "CH93 0076 2011 6238 5295 7",
        },
        RegistryExample {
            country_code: "CR",
            bank_identifier: Some("0152"),
            branch_identifier: None,
            bban: "15202001026284066",
            iban_electronic: "CR05015202001026284066",
            iban_print: "CR05 0152 0200 1026 2840 66",
        },
        RegistryExample {
            country_code: "CY",
            bank_identifier: Some("002"),
            branch_identifier: Some("00128"),
            bban: "002001280000001200527600",
            iban_electronic: "CY17002001280000001200527600",
            iban_print: "CY17 0020 0128 0000 0012 0052 7600",
        },
        RegistryExample {
            country_code: "CZ",
            bank_identifier: Some("0800"),
            branch_identifier: None,
            bban: "08000000192000145399",
            iban_electronic: "CZ6508000000192000145399",
            iban_print: "CZ65 0800 0000 1920 0014 5399",
        },
        RegistryExample {
            country_code: "DE",
            bank_identifier: Some("37040044"),
            branch_identifier: None,
            bban: "370400440532013000",
            iban_electronic: "DE89370400440532013000",
            iban_print: "DE89 3704 0044 0532 0130 00",
        },
        RegistryExample {
            country_code: "DK",
            bank_identifier: Some("0040"),
            branch_identifier: None,
            bban: "00400440116243",
            iban_electronic: "DK5000400440116243",
            iban_print: "DK50 0040 0440 1162 43",
        },
        RegistryExample {
            country_code: "DO",
            bank_identifier: Some("BAGR"),
            branch_identifier: None,
            bban: "BAGR00000001212453611324",
            iban_electronic: "DO28BAGR00000001212453611324",
            iban_print: "DO28 BAGR 0000 0001 2124 5361 1324",
        },
        RegistryExample {
            country_code: "EE",
            bank_identifier: Some("22"),
            branch_identifier: None,
            bban: "2200221020145685",
            iban_electronic: "EE382200221020145685",
            iban_print: "EE38 2200 2210 2014 5685",
        },
        RegistryExample {
            country_code: "EG",
            bank_identifier: Some("0019"),
            branch_identifier: Some("0005"),
            bban: "0019000500000000263180002",
            iban_electronic: "EG380019000500000000263180002",
            iban_print: "EG380019000500000000263180002",
        },
        RegistryExample {
            country_code: "ES",
            bank_identifier: Some("2100"),
            branch_identifier: Some("0418"),
            bban: "21000418450200051332",
            iban_electronic: "ES9121000418450200051332",
            iban_print: "ES91 2100 0418 4502 0005 1332",
        },
        RegistryExample {
            country_code: "FI",
            bank_identifier: Some("123"),
            branch_identifier: None,
            bban: "N/A",
            iban_electronic: "FI2112345600000785",
            iban_print: "FI21 1234 5600 0007 85",
        },
        RegistryExample {
            country_code: "FO",
            bank_identifier: Some("6460"),
            branch_identifier: None,
            bban: "64600001631634",
            iban_electronic: "FO6264600001631634",
            iban_print: "FO62 6460 0001 6316 34",
        },
        RegistryExample {
            country_code: "FR",
            bank_identifier: Some("20041"),
            branch_identifier: None,
            bban: "20041010050500013M02606",
            iban_electronic: "FR1420041010050500013M02606",
            iban_print: "FR14 2004 1010 0505 0001 3M02 606",
        },
        RegistryExample {
            country_code: "GB",
            bank_identifier: Some("NWBK"),
            branch_identifier: Some("601613"),
            bban: "NWBK60161331926819",
            iban_electronic: "GB29NWBK60161331926819",
            iban_print: "GB29 NWBK 6016 1331 9268 19",
        },
        RegistryExample {
            country_code: "GE",
            bank_identifier: Some("NB"),
            branch_identifier: None,
            bban: "NB0000000101904917",
            iban_electronic: "GE29NB0000000101904917",
            iban_print: "GE29 NB00 0000 0101 9049 17",
        },
        RegistryExample {
            country_code: "GI",
            bank_identifier: Some("NWBK"),
            branch_identifier: None,
            bban: "NWBK000000007099453",
            iban_electronic: "GI75NWBK000000007099453",
            iban_print: "GI75 NWBK 0000 0000 7099 453",
        },
        RegistryExample {
            country_code: "GL",
            bank_identifier: Some("6471"),
            branch_identifier: None,
            bban: "64710001000206",
            iban_electronic: "GL8964710001000206",
            iban_print: "GL89 6471 0001 0002 06",
        },
        RegistryExample {
            country_code: "GR",
            bank_identifier: Some("011"),
            branch_identifier: Some("0125"),
            bban: "01101250000000012300695",
            iban_electronic: "GR1601101250000000012300695",
            iban_print: "GR16 0110 1250 0000 0001 2300 695",
        },
        RegistryExample {
            country_code: "GT",
            bank_identifier: Some("TRAJ"),
            branch_identifier: None,
            bban: "TRAJ01020000001210029690",
            iban_electronic: "GT82TRAJ01020000001210029690",
            iban_print: "GT82 TRAJ 0102 0000 0012 1002 9690",
        },
        RegistryExample {
            country_code: "HR",
            bank_identifier: Some("1001005"),
            branch_identifier: None,
            bban: "10010051863000160",
            iban_electronic: "HR1210010051863000160",
            iban_print: "HR12 1001 0051 8630 0016 0",
        },
        RegistryExample {
            country_code: "HU",
            bank_identifier: Some("117"),
            branch_identifier: Some("7301"),
            bban: "117730161111101800000000",
            iban_electronic: "HU42117730161111101800000000",
            iban_print: "HU42 1177 3016 1111 1018 0000 0000",
        },
        RegistryExample {
            country_code: "IE",
            bank_identifier: Some("AIBK"),
            branch_identifier: Some("931152"),
            bban: "AIBK93115212345678",
            iban_electronic: "IE29AIBK93115212345678",
            iban_print: "IE29 AIBK 9311 5212 3456 78",
        },
        RegistryExample {
            country_code: "IL",
            bank_identifier: Some("010"),
            branch_identifier: Some("800"),
            bban: "010800000099999999",
            iban_electronic: "IL620108000000099999999",
            iban_print: "IL62 0108 0000 0009 9999 999",
        },
        RegistryExample {
            country_code: "IQ",
            bank_identifier: Some("NBIQ"),
            branch_identifier: Some("850"),
            bban: "NBIQ850123456789012",
            iban_electronic: "IQ98NBIQ850123456789012",
            iban_print: "IQ98 NBIQ 8501 2345 6789 012",
        },
        RegistryExample {
            country_code: "IS",
            bank_identifier: Some("01"),
            branch_identifier: Some("59"),
            bban: "0159260076545510730339",
            iban_electronic: "IS140159260076545510730339",
            iban_print: "IS14 0159 2600 7654 5510 7303 39",
        },
        RegistryExample {
            country_code: "IT",
            bank_identifier: Some("05428"),
            branch_identifier: Some("11101"),
            bban: "X0542811101000000123456",
            iban_electronic: "IT60X0542811101000000123456",
            iban_print: "IT60 X054 2811 1010 0000 0123 456",
        },
        RegistryExample {
            country_code: "JO",
            bank_identifier: Some("CBJO"),
            branch_identifier: None,
            bban: "CBJO0010000000000131000302",
            iban_electronic: "JO94CBJO0010000000000131000302",
            iban_print: "JO94 CBJO 0010 0000 0000 0131 0003 02",
        },
        RegistryExample {
            country_code: "KW",
            bank_identifier: Some("CBKU"),
            branch_identifier: None,
            bban: "CBKU0000000000001234560101",
            iban_electronic: "KW81CBKU0000000000001234560101",
            iban_print: "KW81 CBKU 0000 0000 0000 1234 5601 01",
        },
        RegistryExample {
            country_code: "KZ",
            bank_identifier: Some("125"),
            branch_identifier: None,
            bban: "125KZT5004100100",
            iban_electronic: "KZ86125KZT5004100100",
            iban_print: "KZ86 125K ZT50 0410 0100",
        },
        RegistryExample {
            country_code: "LB",
            bank_identifier: Some("0999"),
            branch_identifier: None,
            bban: "0999 0000 0001 0019 0122 9114",
            iban_electronic: "LB62099900000001001901229114",
            iban_print: "LB62 0999 0000 0001 0019 0122 9114",
        },
        RegistryExample {
            country_code: "LC",
            bank_identifier: Some("HEMM"),
            branch_identifier: None,
            bban: "HEMM000100010012001200023015",
            iban_electronic: "LC55HEMM000100010012001200023015",
            iban_print: "LC55 HEMM 0001 0001 0012 0012 0002 3015",
        },
        RegistryExample {
            country_code: "LI",
            bank_identifier: Some("08810"),
            branch_identifier: None,
            bban: "088100002324013AA",
            iban_electronic: "LI21088100002324013AA",
            iban_print: "LI21 0881 0000 2324 013A A",
        },
        RegistryExample {
            country_code: "LT",
            bank_identifier: Some("10000"),
            branch_identifier: None,
            bban: "1000011101001000",
            iban_electronic: "LT121000011101001000",
            iban_print: "LT12 1000 0111 0100 1000",
        },
        RegistryExample {
            country_code: "LU",
            bank_identifier: Some("001"),
            branch_identifier: None,
            bban: "0019400644750000",
            iban_electronic: "LU280019400644750000",
            iban_print: "LU28 0019 4006 4475 0000",
        },
        RegistryExample {
            country_code: "LV",
            bank_identifier: Some("BANK"),
            branch_identifier: None,
            bban: "BANK0000435195001",
            iban_electronic: "LV80BANK0000435195001",
            iban_print: "LV80 BANK 0000 4351 9500 1",
        },
        RegistryExample {
            country_code: "LY",
            bank_identifier: Some("002"),
            branch_identifier: Some("048"),
            bban: "002048000020100120361",
            iban_electronic: "LY83002048000020100120361",
            iban_print: "LY83 002 048 000020100120361",
        },
        RegistryExample {
            country_code: "MC",
            bank_identifier: Some("11222"),
            branch_identifier: Some("00001"),
            bban: "11222 00001 01234567890 30",
            iban_electronic: "MC5811222000010123456789030",
            iban_print: "MC58 1122 2000 0101 2345 6789 030",
        },
        RegistryExample {
            country_code: "MD",
            bank_identifier: Some("AG"),
            branch_identifier: None,
            bban: "AG000225100013104168",
            iban_electronic: "MD24AG000225100013104168",
            iban_print: "MD24 AG00 0225 1000 1310 4168",
        },
        RegistryExample {
            country_code: "ME",
            bank_identifier: Some("505"),
            branch_identifier: None,
            bban: "505000012345678951",
            iban_electronic: "ME25505000012345678951",
            iban_print: "ME25 5050 0001 2345 6789 51",
        },
        RegistryExample {
            country_code: "MK",
            bank_identifier: Some("300"),
            branch_identifier: None,
            bban: "250120000058984",
            iban_electronic: "MK07250120000058984",
            iban_print: "MK07 2501 2000 0058 984",
        },
        RegistryExample {
            country_code: "MR",
            bank_identifier: Some("00020"),
            branch_identifier: Some("00101"),
            bban: "00020001010000123456753",
            iban_electronic: "MR1300020001010000123456753",
            iban_print: "MR13 0002 0001 0100 0012 3456 753",
        },
        RegistryExample {
            country_code: "MT",
            bank_identifier: Some("MALT"),
            branch_identifier: Some("01100"),
            bban: "MALT011000012345MTLCAST001S",
            iban_electronic: "MT84MALT011000012345MTLCAST001S",
            iban_print: "MT84 MALT 0110 0001 2345 MTLC AST0 01S",
        },
        RegistryExample {
            country_code: "MU",
            bank_identifier: Some("BOMM01"),
            branch_identifier: Some("01"),
            bban: "BOMM0101101030300200000MUR",
            iban_electronic: "MU17BOMM0101101030300200000MUR",
            iban_print: "MU17 BOMM 0101 1010 3030 0200 000M UR",
        },
        RegistryExample {
            country_code: "NL",
            bank_identifier: Some("ABNA"),
            branch_identifier: None,
            bban: "ABNA0417164300",
            iban_electronic: "NL91ABNA0417164300",
            iban_print: "NL91 ABNA 0417 1643 00",
        },
        RegistryExample {
            country_code: "NO",
            bank_identifier: Some("8601"),
            branch_identifier: None,
            bban: "86011117947",
            iban_electronic: "NO9386011117947",
            iban_print: "NO93 8601 1117 947",
        },
        RegistryExample {
            country_code: "PK",
            bank_identifier: Some("SCBL"),
            branch_identifier: None,
            bban: "SCBL0000001123456702",
            iban_electronic: "PK36SCBL0000001123456702",
            iban_print: "PK36 SCBL 0000 0011 2345 6702",
        },
        RegistryExample {
            country_code: "PL",
            bank_identifier: None,
            branch_identifier: Some("10901014"),
            bban: "109010140000071219812874",
            iban_electronic: "PL61109010140000071219812874",
            iban_print: "PL61 1090 1014 0000 0712 1981 2874",
        },
        RegistryExample {
            country_code: "PS",
            bank_identifier: Some("PALS"),
            branch_identifier: None,
            bban: "PALS000000000400123456702",
            iban_electronic: "PS92PALS000000000400123456702",
            iban_print: "PS92 PALS 0000 0000 0400 1234 5670 2",
        },
        RegistryExample {
            country_code: "PT",
            bank_identifier: Some("0002"),
            branch_identifier: None,
            bban: "000201231234567890154",
            iban_electronic: "PT50000201231234567890154",
            iban_print: "PT50 0002 0123 1234 5678 9015 4",
        },
        RegistryExample {
            country_code: "QA",
            bank_identifier: Some("DOHB"),
            branch_identifier: None,
            bban: "DOHB00001234567890ABCDEFG",
            iban_electronic: "QA58DOHB00001234567890ABCDEFG",
            iban_print: "QA58 DOHB 0000 1234 5678 90AB CDEF G",
        },
        RegistryExample {
            country_code: "RO",
            bank_identifier: Some("AAAA"),
            branch_identifier: None,
            bban: "AAAA1B31007593840000",
            iban_electronic: "RO49AAAA1B31007593840000",
            iban_print: "RO49 AAAA 1B31 0075 9384 0000",
        },
        RegistryExample {
            country_code: "RS",
            bank_identifier: Some("260"),
            branch_identifier: None,
            bban: "260005601001611379",
            iban_electronic: "RS35260005601001611379",
            iban_print: "RS35 2600 0560 1001 6113 79",
        },
        RegistryExample {
            country_code: "SA",
            bank_identifier: Some("80"),
            branch_identifier: None,
            bban: "80000000608010167519",
            iban_electronic: "SA0380000000608010167519",
            iban_print: "SA03 8000 0000 6080 1016 7519",
        },
        RegistryExample {
            country_code: "SC",
            bank_identifier: Some("SSCB11"),
            branch_identifier: Some("01"),
            bban: "SSCB11010000000000001497USD",
            iban_electronic: "SC18SSCB11010000000000001497USD",
            iban_print: "SC18 SSCB 1101 0000 0000 0000 1497 USD",
        },
        RegistryExample {
            country_code: "SD",
            bank_identifier: Some("29"),
            branch_identifier: None,
            bban: "29010501234001",
            iban_electronic: "SD2129010501234001",
            iban_print: "SD21 2901 0501 2340 01",
        },
        RegistryExample {
            country_code: "SE",
            bank_identifier: Some("123"),
            branch_identifier: None,
            bban: "50000000058398257466",
            iban_electronic: "SE4550000000058398257466",
            iban_print: "SE45 5000 0000 0583 9825 7466",
        },
        RegistryExample {
            country_code: "SI",
            bank_identifier: Some("26330"),
            branch_identifier: None,
            bban: "263300012039086",
            iban_electronic: "SI56263300012039086",
            iban_print: "SI56 2633 0001 2039 086",
        },
        RegistryExample {
            country_code: "SK",
            bank_identifier: Some("1200"),
            branch_identifier: None,
            bban: "12000000198742637541",
            iban_electronic: "SK3112000000198742637541",
            iban_print: "SK31 1200 0000 1987 4263 7541",
        },
        RegistryExample {
            country_code: "SM",
            bank_identifier: Some("03225"),
            branch_identifier: Some("09800"),
            bban: "U0322509800000000270100",
            iban_electronic: "SM86U0322509800000000270100",
            iban_print: "SM86 U032 2509 8000 0000 0270 100",
        },
        RegistryExample {
            country_code: "ST",
            bank_identifier: Some("0001"),
            branch_identifier: Some("0001"),
            bban: "000200010192194210112",
            iban_electronic: "ST68000200010192194210112",
            iban_print: "ST68 0002 0001 0192 1942 1011 2",
        },
        RegistryExample {
            country_code: "SV",
            bank_identifier: Some("CENR"),
            branch_identifier: None,
            bban: "CENR00000000000000700025",
            iban_electronic: "SV62CENR00000000000000700025",
            iban_print: "SV 62 CENR 00000000000000700025",
        },
        RegistryExample {
            country_code: "TL",
            bank_identifier: Some("008"),
            branch_identifier: None,
            bban: "0080012345678910157",
            iban_electronic: "TL380080012345678910157",
            iban_print: "TL38 0080 0123 4567 8910 157",
        },
        RegistryExample {
            country_code: "TN",
            bank_identifier: Some("10"),
            branch_identifier: Some("006"),
            bban: "10006035183598478831",
            iban_electronic: "TN5910006035183598478831",
            iban_print: "TN59 1000 6035 1835 9847 8831",
        },
        RegistryExample {
            country_code: "TR",
            bank_identifier: Some("00061"),
            branch_identifier: None,
            bban: "0006100519786457841326",
            iban_electronic: "TR330006100519786457841326",
            iban_print: "TR33 0006 1005 1978 6457 8413 26",
        },
        RegistryExample {
            country_code: "UA",
            bank_identifier: Some("322313"),
            branch_identifier: None,
            bban: "3223130000026007233566001",
            iban_electronic: "UA213223130000026007233566001",
            iban_print: "UA21 3223 1300 0002 6007 2335 6600 1",
        },
        RegistryExample {
            country_code: "VA",
            bank_identifier: Some("001"),
            branch_identifier: None,
            bban: "001123000012345678",
            iban_electronic: "VA59001123000012345678",
            iban_print: "VA59 001 1230 0001 2345 678",
        },
        RegistryExample {
            country_code: "VG",
            bank_identifier: Some("VPVG"),
            branch_identifier: None,
            bban: "VPVG0000012345678901",
            iban_electronic: "VG96VPVG0000012345678901",
            iban_print: "VG96 VPVG 0000 0123 4567 8901",
        },
        RegistryExample {
            country_code: "XK",
            bank_identifier: Some("12"),
            branch_identifier: Some("12"),
            bban: "1212012345678906",
            iban_electronic: "XK051212012345678906",
            iban_print: "XK05 1212 0123 4567 8906",
        },
    ];

    for RegistryExample {
        country_code,
        bank_identifier,
        branch_identifier,
        bban,
        iban_electronic,
        iban_print,
    } in examples
    {
        if *country_code == "ST" {
            // Incredibly, the IBAN for ST actually has an incorrect checksum. Yes, the example of a correct IBAN is incorrect.
            continue;
        }

        let iban_1 = iban_electronic.parse::<Iban>().unwrap_or_else(|_| {
            panic!(
                "could not parse electronic format of country {}",
                country_code
            )
        });

        // For the countries that abide by the pretty print format, check if the parsed IBAN is identical.
        // We could remove the whitespace and parse again, but that's probably not worth it.
        if !matches!(*country_code, "BI" | "LY" | "SV" | "VA") {
            let iban_2 = iban_print.parse::<Iban>().unwrap_or_else(|_| {
                panic!("could not parse print format of country {}", country_code)
            });
            assert_eq!(
                iban_1, iban_2,
                "parsed ibans should be identical, regardless of format"
            );
        }

        // Validate the country code.
        assert_eq!(
            iban_1.country_code(),
            *country_code,
            "country codes do not match for country {}",
            country_code
        );

        // Validate the bank identifier.
        let bank_identifier: Option<String> =
            bank_identifier.map(|c| c.chars().filter(|c| c.is_ascii_alphanumeric()).collect());
        if matches!(*country_code, "BA" | "MK" | "SE") {
            // For these countries, the examples do not match. Test the length instead.
            assert_eq!(
                iban_1.bank_identifier().unwrap().len(),
                bank_identifier.unwrap().len(),
                "bank identifier lengths do not match for country {}",
                country_code
            );
        } else {
            assert_eq!(
                iban_1.bank_identifier(),
                bank_identifier.as_deref(),
                "bank identifiers do not match for country {}",
                country_code
            );
        }

        // Validate the branch identifier.
        if *country_code == "JO" {
            // There is no example here.
        } else if matches!(*country_code, "BA") {
            // For these countries, the examples do not match. Test the length instead.
            assert_eq!(
                iban_1.branch_identifier().unwrap().len(),
                branch_identifier.unwrap().len(),
                "branch identifier lengths do not match for country {}",
                country_code
            );
        } else {
            assert_eq!(
                iban_1.branch_identifier(),
                *branch_identifier,
                "branch identifiers do not match for country {}",
                country_code
            );
        }

        // Validate the BBAN.
        let bban: String = bban
            .chars()
            .filter(|c: &char| c.is_ascii_alphanumeric())
            .collect();
        if !matches!(*country_code, "BA" | "BR" | "CR" | "FI" | "IL") {
            assert_eq!(
                iban_1.bban(),
                bban,
                "the bban doesn't match for country {}",
                country_code
            );
        }
    }
    Ok(())
}
