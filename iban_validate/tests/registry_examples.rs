//! Use the registry examples to validate the code. As you can see, there are a lot of errors/exceptions, unfortunately.

use iban::{Iban, IbanLike, ParseIbanError};

#[test]
fn test_registry_examples() -> Result<(), ParseIbanError> {
    //
    let examples: &[(&str, Option<&str>, Option<&str>, &str, &str, &str)] = &[
        (
            "AD",
            Some("0001"),
            Some("2030"),
            "00012030200359100100",
            "AD1200012030200359100100",
            "AD12 0001 2030 2003 5910 0100",
        ),
        (
            "AE",
            Some("033"),
            None,
            "0331234567890123456",
            "AE070331234567890123456",
            "AE07 0331 2345 6789 0123 456",
        ),
        (
            "AL",
            Some("21211009"),
            Some("1100"),
            "212110090000000235698741",
            "AL47212110090000000235698741",
            "AL47 2121 1009 0000 0002 3569 8741",
        ),
        (
            "AT",
            Some("19043"),
            None,
            "1904300234573201",
            "AT611904300234573201",
            "AT61 1904 3002 3457 3201",
        ),
        (
            "AZ",
            Some("NABZ"),
            None,
            "NABZ00000000137010001944",
            "AZ21NABZ00000000137010001944",
            "AZ21 NABZ 0000 0000 1370 1000 1944",
        ),
        (
            "BA",
            Some("199"),
            Some("044"),
            "1990440001200279",
            "BA391290079401028494",
            "BA39 1290 0794 0102 8494",
        ),
        (
            "BE",
            Some("539"),
            None,
            "539007547034",
            "BE68539007547034",
            "BE68 5390 0754 7034",
        ),
        (
            "BG",
            Some("BNBG"),
            Some("9661"),
            "BNBG96611020345678",
            "BG80BNBG96611020345678",
            "BG80 BNBG 9661 1020 3456 78",
        ),
        (
            "BH",
            Some("BMAG"),
            None,
            "BMAG00001299123456",
            "BH67BMAG00001299123456",
            "BH67 BMAG 0000 1299 1234 56",
        ),
        (
            "BI",
            Some("10000"),
            Some("10001"),
            "10000100010000332045181",
            "BI4210000100010000332045181",
            "BI42 10000 10001 00003320451 81",
        ),
        (
            "BR",
            Some("00360305"),
            Some("00001"),
            "00360305000010009795493P1",
            "BR1800360305000010009795493C1",
            "BR18 0036 0305 0000 1000 9795 493C 1",
        ),
        (
            "BY",
            Some("NBRB"),
            None,
            "NBRB 3600900000002Z00AB00",
            "BY13NBRB3600900000002Z00AB00",
            "BY13 NBRB 3600 9000 0000 2Z00 AB00",
        ),
        (
            "CH",
            Some("00762"),
            None,
            "00762011623852957",
            "CH9300762011623852957",
            "CH93 0076 2011 6238 5295 7",
        ),
        (
            "CR",
            Some("0152"),
            None,
            "15202001026284066",
            "CR05015202001026284066",
            "CR05 0152 0200 1026 2840 66",
        ),
        (
            "CY",
            Some("002"),
            Some("00128"),
            "002001280000001200527600",
            "CY17002001280000001200527600",
            "CY17 0020 0128 0000 0012 0052 7600",
        ),
        (
            "CZ",
            Some("0800"),
            None,
            "08000000192000145399",
            "CZ6508000000192000145399",
            "CZ65 0800 0000 1920 0014 5399",
        ),
        (
            "DE",
            Some("37040044"),
            None,
            "370400440532013000",
            "DE89370400440532013000",
            "DE89 3704 0044 0532 0130 00",
        ),
        (
            "DK",
            Some("0040"),
            None,
            "00400440116243",
            "DK5000400440116243",
            "DK50 0040 0440 1162 43",
        ),
        (
            "DO",
            Some("BAGR"),
            None,
            "BAGR00000001212453611324",
            "DO28BAGR00000001212453611324",
            "DO28 BAGR 0000 0001 2124 5361 1324",
        ),
        (
            "EE",
            Some("22"),
            None,
            "2200221020145685",
            "EE382200221020145685",
            "EE38 2200 2210 2014 5685",
        ),
        (
            "EG",
            Some("0019"),
            Some("0005"),
            "0019000500000000263180002",
            "EG380019000500000000263180002",
            "EG380019000500000000263180002",
        ),
        (
            "ES",
            Some("2100"),
            Some("0418"),
            "21000418450200051332",
            "ES9121000418450200051332",
            "ES91 2100 0418 4502 0005 1332",
        ),
        (
            "FI",
            Some("123"),
            None,
            "N/A",
            "FI2112345600000785",
            "FI21 1234 5600 0007 85",
        ),
        (
            "FO",
            Some("6460"),
            None,
            "64600001631634",
            "FO6264600001631634",
            "FO62 6460 0001 6316 34",
        ),
        (
            "FR",
            Some("20041"),
            None,
            "20041010050500013M02606",
            "FR1420041010050500013M02606",
            "FR14 2004 1010 0505 0001 3M02 606",
        ),
        (
            "GB",
            Some("NWBK"),
            Some("601613"),
            "NWBK60161331926819",
            "GB29NWBK60161331926819",
            "GB29 NWBK 6016 1331 9268 19",
        ),
        (
            "GE",
            Some("NB"),
            None,
            "NB0000000101904917",
            "GE29NB0000000101904917",
            "GE29 NB00 0000 0101 9049 17",
        ),
        (
            "GI",
            Some("NWBK"),
            None,
            "NWBK000000007099453",
            "GI75NWBK000000007099453",
            "GI75 NWBK 0000 0000 7099 453",
        ),
        (
            "GL",
            Some("6471"),
            None,
            "64710001000206",
            "GL8964710001000206",
            "GL89 6471 0001 0002 06",
        ),
        (
            "GR",
            Some("011"),
            Some("0125"),
            "01101250000000012300695",
            "GR1601101250000000012300695",
            "GR16 0110 1250 0000 0001 2300 695",
        ),
        (
            "GT",
            Some("TRAJ"),
            None,
            "TRAJ01020000001210029690",
            "GT82TRAJ01020000001210029690",
            "GT82 TRAJ 0102 0000 0012 1002 9690",
        ),
        (
            "HR",
            Some("1001005"),
            None,
            "10010051863000160",
            "HR1210010051863000160",
            "HR12 1001 0051 8630 0016 0",
        ),
        (
            "HU",
            Some("117"),
            Some("7301"),
            "117730161111101800000000",
            "HU42117730161111101800000000",
            "HU42 1177 3016 1111 1018 0000 0000",
        ),
        (
            "IE",
            Some("AIBK"),
            Some("931152"),
            "AIBK93115212345678",
            "IE29AIBK93115212345678",
            "IE29 AIBK 9311 5212 3456 78",
        ),
        (
            "IL",
            Some("010"),
            Some("800"),
            "010800000099999999",
            "IL620108000000099999999",
            "IL62 0108 0000 0009 9999 999",
        ),
        (
            "IQ",
            Some("NBIQ"),
            Some("850"),
            "NBIQ850123456789012",
            "IQ98NBIQ850123456789012",
            "IQ98 NBIQ 8501 2345 6789 012",
        ),
        (
            "IS",
            Some("01"),
            Some("59"),
            "0159260076545510730339",
            "IS140159260076545510730339",
            "IS14 0159 2600 7654 5510 7303 39",
        ),
        (
            "IT",
            Some("05428"),
            Some("11101"),
            "X0542811101000000123456",
            "IT60X0542811101000000123456",
            "IT60 X054 2811 1010 0000 0123 456",
        ),
        (
            "JO",
            Some("CBJO"),
            None,
            "CBJO0010000000000131000302",
            "JO94CBJO0010000000000131000302",
            "JO94 CBJO 0010 0000 0000 0131 0003 02",
        ),
        (
            "KW",
            Some("CBKU"),
            None,
            "CBKU0000000000001234560101",
            "KW81CBKU0000000000001234560101",
            "KW81 CBKU 0000 0000 0000 1234 5601 01",
        ),
        (
            "KZ",
            Some("125"),
            None,
            "125KZT5004100100",
            "KZ86125KZT5004100100",
            "KZ86 125K ZT50 0410 0100",
        ),
        (
            "LB",
            Some("0999"),
            None,
            "0999 0000 0001 0019 0122 9114",
            "LB62099900000001001901229114",
            "LB62 0999 0000 0001 0019 0122 9114",
        ),
        (
            "LC",
            Some("HEMM"),
            None,
            "HEMM000100010012001200023015",
            "LC55HEMM000100010012001200023015",
            "LC55 HEMM 0001 0001 0012 0012 0002 3015",
        ),
        (
            "LI",
            Some("08810"),
            None,
            "088100002324013AA",
            "LI21088100002324013AA",
            "LI21 0881 0000 2324 013A A",
        ),
        (
            "LT",
            Some("10000"),
            None,
            "1000011101001000",
            "LT121000011101001000",
            "LT12 1000 0111 0100 1000",
        ),
        (
            "LU",
            Some("001"),
            None,
            "0019400644750000",
            "LU280019400644750000",
            "LU28 0019 4006 4475 0000",
        ),
        (
            "LV",
            Some("BANK"),
            None,
            "BANK0000435195001",
            "LV80BANK0000435195001",
            "LV80 BANK 0000 4351 9500 1",
        ),
        (
            "LY",
            Some("002"),
            Some("048"),
            "002048000020100120361",
            "LY83002048000020100120361",
            "LY83 002 048 000020100120361",
        ),
        (
            "MC",
            Some("11222"),
            Some("00001"),
            "11222 00001 01234567890 30",
            "MC5811222000010123456789030",
            "MC58 1122 2000 0101 2345 6789 030",
        ),
        (
            "MD",
            Some("AG"),
            None,
            "AG000225100013104168",
            "MD24AG000225100013104168",
            "MD24 AG00 0225 1000 1310 4168",
        ),
        (
            "ME",
            Some("505"),
            None,
            "505000012345678951",
            "ME25505000012345678951",
            "ME25 5050 0001 2345 6789 51",
        ),
        (
            "MK",
            Some("300"),
            None,
            "250120000058984",
            "MK07250120000058984",
            "MK07 2501 2000 0058 984",
        ),
        (
            "MR",
            Some("00020"),
            Some("00101"),
            "00020001010000123456753",
            "MR1300020001010000123456753",
            "MR13 0002 0001 0100 0012 3456 753",
        ),
        (
            "MT",
            Some("MALT"),
            Some("01100"),
            "MALT011000012345MTLCAST001S",
            "MT84MALT011000012345MTLCAST001S",
            "MT84 MALT 0110 0001 2345 MTLC AST0 01S",
        ),
        (
            "MU",
            Some("BOMM01"),
            Some("01"),
            "BOMM0101101030300200000MUR",
            "MU17BOMM0101101030300200000MUR",
            "MU17 BOMM 0101 1010 3030 0200 000M UR",
        ),
        (
            "NL",
            Some("ABNA"),
            None,
            "ABNA0417164300",
            "NL91ABNA0417164300",
            "NL91 ABNA 0417 1643 00",
        ),
        (
            "NO",
            Some("8601"),
            None,
            "86011117947",
            "NO9386011117947",
            "NO93 8601 1117 947",
        ),
        (
            "PK",
            Some("SCBL"),
            None,
            "SCBL0000001123456702",
            "PK36SCBL0000001123456702",
            "PK36 SCBL 0000 0011 2345 6702",
        ),
        (
            "PL",
            None,
            Some("10901014"),
            "109010140000071219812874",
            "PL61109010140000071219812874",
            "PL61 1090 1014 0000 0712 1981 2874",
        ),
        (
            "PS",
            Some("PALS"),
            None,
            "PALS000000000400123456702",
            "PS92PALS000000000400123456702",
            "PS92 PALS 0000 0000 0400 1234 5670 2",
        ),
        (
            "PT",
            Some("0002"),
            None,
            "000201231234567890154",
            "PT50000201231234567890154",
            "PT50 0002 0123 1234 5678 9015 4",
        ),
        (
            "QA",
            Some("DOHB"),
            None,
            "DOHB00001234567890ABCDEFG",
            "QA58DOHB00001234567890ABCDEFG",
            "QA58 DOHB 0000 1234 5678 90AB CDEF G",
        ),
        (
            "RO",
            Some("AAAA"),
            None,
            "AAAA1B31007593840000",
            "RO49AAAA1B31007593840000",
            "RO49 AAAA 1B31 0075 9384 0000",
        ),
        (
            "RS",
            Some("260"),
            None,
            "260005601001611379",
            "RS35260005601001611379",
            "RS35 2600 0560 1001 6113 79",
        ),
        (
            "SA",
            Some("80"),
            None,
            "80000000608010167519",
            "SA0380000000608010167519",
            "SA03 8000 0000 6080 1016 7519",
        ),
        (
            "SC",
            Some("SSCB11"),
            Some("01"),
            "SSCB11010000000000001497USD",
            "SC18SSCB11010000000000001497USD",
            "SC18 SSCB 1101 0000 0000 0000 1497 USD",
        ),
        (
            "SD",
            Some("29"),
            None,
            "29010501234001",
            "SD2129010501234001",
            "SD21 2901 0501 2340 01",
        ),
        (
            "SE",
            Some("123"),
            None,
            "50000000058398257466",
            "SE4550000000058398257466",
            "SE45 5000 0000 0583 9825 7466",
        ),
        (
            "SI",
            Some("26330"),
            None,
            "263300012039086",
            "SI56263300012039086",
            "SI56 2633 0001 2039 086",
        ),
        (
            "SK",
            Some("1200"),
            None,
            "12000000198742637541",
            "SK3112000000198742637541",
            "SK31 1200 0000 1987 4263 7541",
        ),
        (
            "SM",
            Some("03225"),
            Some("09800"),
            "U0322509800000000270100",
            "SM86U0322509800000000270100",
            "SM86 U032 2509 8000 0000 0270 100",
        ),
        (
            "ST",
            Some("0001"),
            Some("0001"),
            "000200010192194210112",
            "ST68000200010192194210112",
            "ST68 0002 0001 0192 1942 1011 2 ",
        ),
        (
            "SV",
            Some("CENR"),
            None,
            "CENR00000000000000700025",
            "SV62CENR00000000000000700025",
            "SV 62 CENR 00000000000000700025",
        ),
        (
            "TL",
            Some("008"),
            None,
            "0080012345678910157",
            "TL380080012345678910157",
            "TL38 0080 0123 4567 8910 157",
        ),
        (
            "TN",
            Some("10"),
            Some("006"),
            "10006035183598478831",
            "TN5910006035183598478831",
            "TN59 1000 6035 1835 9847 8831",
        ),
        (
            "TR",
            Some("00061"),
            None,
            "0006100519786457841326",
            "TR330006100519786457841326",
            "TR33 0006 1005 1978 6457 8413 26",
        ),
        (
            "UA",
            Some("322313"),
            None,
            "3223130000026007233566001",
            "UA213223130000026007233566001",
            "UA21 3223 1300 0002 6007 2335 6600 1",
        ),
        (
            "VA",
            Some("001"),
            None,
            "001123000012345678",
            "VA59001123000012345678",
            "VA59 001 1230 0001 2345 678",
        ),
        (
            "VG",
            Some("VPVG"),
            None,
            "VPVG0000012345678901",
            "VG96VPVG0000012345678901",
            "VG96 VPVG 0000 0123 4567 8901",
        ),
        (
            "XK",
            Some("12"),
            Some("12"),
            "1212012345678906",
            "XK051212012345678906",
            "XK05 1212 0123 4567 8906",
        ),
    ];

    for (country_code, bank_identifier, branch_identifier, bban, iban_electronic, iban_print) in
        examples
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
                *bank_identifier,
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
