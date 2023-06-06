#[must_use]
pub fn is(s: &str) -> bool {
    REGISTRATION_AUTHORITIES.contains(&s)
}

/// extracted from 2019-12-05_ra-list-v1.5
const REGISTRATION_AUTHORITIES: [&str; 737] = [
    "RA888888", "RA999999", "RA000001", "RA000002", "RA000003", "RA000004", "RA000661", "RA000005",
    "RA000006", "RA000007", "RA000008", "RA000009", "RA000010", "RA000011", "RA000012", "RA000013",
    "RA000014", "RA000015", "RA000016", "RA000714", "RA000017", "RA000018", "RA000687", "RA000707",
    "RA000710", "RA000725", "RA000734", "RA000019", "RA000020", "RA000021", "RA000022", "RA000023",
    "RA000024", "RA000025", "RA000026", "RA000693", "RA000027", "RA000028", "RA000029", "RA000030",
    "RA000031", "RA000032", "RA000033", "RA000034", "RA000692", "RA000035", "RA000062", "RA000036",
    "RA000037", "RA000038", "RA000039", "RA000681", "RA000040", "RA000041", "RA000042", "RA000043",
    "RA000044", "RA000046", "RA000045", "RA000047", "RA000048", "RA000049", "RA000050", "RA000051",
    "RA000052", "RA000054", "RA000055", "RA000053", "RA000056", "RA000057", "RA000059", "RA000058",
    "RA000060", "RA000061", "RA000063", "RA000064", "RA000065", "RA000066", "RA000730", "RA000731",
    "RA000067", "RA000068", "RA000069", "RA000683", "RA000070", "RA000071", "RA000073", "RA000074",
    "RA000072", "RA000075", "RA000076", "RA000077", "RA000083", "RA000078", "RA000084", "RA000079",
    "RA000080", "RA000081", "RA000082", "RA000085", "RA000086", "RA000087", "RA000088", "RA000089",
    "RA000090", "RA000091", "RA000092", "RA000094", "RA000095", "RA000096", "RA000097", "RA000098",
    "RA000099", "RA000100", "RA000101", "RA000102", "RA000103", "RA000104", "RA000110", "RA000105",
    "RA000106", "RA000107", "RA000108", "RA000109", "RA000111", "RA000136", "RA000093", "RA000112",
    "RA000113", "RA000114", "RA000115", "RA000116", "RA000117", "RA000118", "RA000119", "RA000120",
    "RA000121", "RA000122", "RA000123", "RA000124", "RA000125", "RA000126", "RA000127", "RA000128",
    "RA000129", "RA000130", "RA000131", "RA000132", "RA000133", "RA000134", "RA000135", "RA000137",
    "RA000138", "RA000139", "RA000140", "RA000141", "RA000142", "RA000143", "RA000144", "RA000145",
    "RA000146", "RA000147", "RA000148", "RA000149", "RA000150", "RA000151", "RA000152", "RA000153",
    "RA000154", "RA000156", "RA000157", "RA000659", "RA000686", "RA000695", "RA000696", "RA000705",
    "RA000158", "RA000159", "RA000160", "RA000161", "RA000162", "RA000163", "RA000164", "RA000165",
    "RA000166", "RA000167", "RA000168", "RA000694", "RA000155", "RA000169", "RA000170", "RA000171",
    "RA000172", "RA000173", "RA000174", "RA000175", "RA000176", "RA000177", "RA000178", "RA000691",
    "RA000179", "RA000180", "RA000181", "RA000182", "RA000183", "RA000184", "RA000185", "RA000186",
    "RA000187", "RA000188", "RA000189", "RA000190", "RA000192", "RA000189", "RA000192", "RA000189",
    "RA000192", "RA000193", "RA000194", "RA000195", "RA000196", "RA000197", "RA000199", "RA000200",
    "RA000201", "RA000202", "RA000203", "RA000204", "RA000205", "RA000206", "RA000207", "RA000208",
    "RA000209", "RA000210", "RA000213", "RA000214", "RA000724", "RA000215", "RA000704", "RA000216",
    "RA000217", "RA000727", "RA000218", "RA000219", "RA000220", "RA000733", "RA000221", "RA000222",
    "RA000742", "RA000224", "RA000225", "RA000226", "RA000227", "RA000233", "RA000234", "RA000228",
    "RA000229", "RA000230", "RA000712", "RA000231", "RA000232", "RA000235", "RA000236", "RA000238",
    "RA000239", "RA000247", "RA000240", "RA000241", "RA000242", "RA000243", "RA000244", "RA000245",
    "RA000246", "RA000253", "RA000257", "RA000249", "RA000250", "RA000372", "RA000373", "RA000663",
    "RA000251", "RA000252", "RA000255", "RA000258", "RA000259", "RA000738", "RA000260", "RA000261",
    "RA000262", "RA000263", "RA000722", "RA000266", "RA000267", "RA000268", "RA000269", "RA000271",
    "RA000272", "RA000273", "RA000280", "RA000281", "RA000274", "RA000275", "RA000276", "RA000277",
    "RA000278", "RA000279", "RA000282", "RA000283", "RA000290", "RA000293", "RA000284", "RA000285",
    "RA000286", "RA000287", "RA000288", "RA000289", "RA000755", "RA000291", "RA000301", "RA000303",
    "RA000304", "RA000305", "RA000295", "RA000296", "RA000297", "RA000743", "RA000298", "RA000299",
    "RA000300", "RA000302", "RA000311", "RA000306", "RA000307", "RA000308", "RA000309", "RA000310",
    "RA000711", "RA000313", "RA000314", "RA000315", "RA000316", "RA000321", "RA000317", "RA000319",
    "RA000320", "RA000322", "RA000324", "RA000325", "RA000737", "RA000328", "RA000329", "RA000342",
    "RA000331", "RA000741", "RA000332", "RA000333", "RA000334", "RA000740", "RA000739", "RA000723",
    "RA000336", "RA000337", "RA000338", "RA000339", "RA000343", "RA000344", "RA000345", "RA000346",
    "RA000347", "RA000348", "RA000349", "RA000350", "RA000351", "RA000352", "RA000736", "RA000353",
    "RA000354", "RA000355", "RA000356", "RA000370", "RA000358", "RA000362", "RA000363", "RA000364",
    "RA000365", "RA000368", "RA000369", "RA000371", "RA000374", "RA000375", "RA000376", "RA000377",
    "RA000685", "RA000378", "RA000655", "RA000379", "RA000189", "RA000192", "RA000381", "RA000382",
    "RA000383", "RA000666", "RA000667", "RA000668", "RA000384", "RA000385", "RA000386", "RA000387",
    "RA000388", "RA000389", "RA000390", "RA000391", "RA000392", "RA000656", "RA000732", "RA000750",
    "RA000393", "RA000394", "RA000395", "RA000396", "RA000397", "RA000398", "RA000399", "RA000697",
    "RA000708", "RA000709", "RA000713", "RA000717", "RA000754", "RA000400", "RA000680", "RA000401",
    "RA000402", "RA000403", "RA000404", "RA000700", "RA000405", "RA000406", "RA000407", "RA000408",
    "RA000409", "RA000410", "RA000699", "RA000411", "RA000412", "RA000413", "RA000414", "RA000698",
    "RA000415", "RA000416", "RA000417", "RA000418", "RA000519", "RA000420", "RA000421", "RA000422",
    "RA000423", "RA000424", "RA000425", "RA000729", "RA000426", "RA000427", "RA000428", "RA000429",
    "RA000548", "RA000430", "RA000431", "RA000432", "RA000433", "RA000434", "RA000435", "RA000436",
    "RA000679", "RA000438", "RA000440", "RA000439", "RA000441", "RA000442", "RA000443", "RA000444",
    "RA000189", "RA000192", "RA000446", "RA000447", "RA000189", "RA000192", "RA000449", "RA000682",
    "RA000703", "RA000451", "RA000452", "RA000453", "RA000454", "RA000455", "RA000456", "RA000457",
    "RA000458", "RA000459", "RA000460", "RA000461", "RA000462", "RA000463", "RA000464", "RA000664",
    "RA000189", "RA000192", "RA000466", "RA000749", "RA000467", "RA000468", "RA000469", "RA000470",
    "RA000437", "RA000471", "RA000472", "RA000473", "RA000474", "RA000475", "RA000476", "RA000477",
    "RA000478", "RA000479", "RA000480", "RA000481", "RA000482", "RA000689", "RA000483", "RA000484",
    "RA000485", "RA000486", "RA000654", "RA000487", "RA000488", "RA000489", "RA000493", "RA000494",
    "RA000495", "RA000419", "RA000657", "RA000658", "RA000497", "RA000498", "RA000718", "RA000719",
    "RA000720", "RA000721", "RA000499", "RA000500", "RA000189", "RA000192", "RA000189", "RA000192",
    "RA000502", "RA000503", "RA000504", "RA000189", "RA000192", "RA000189", "RA000192", "RA000508",
    "RA000509", "RA000510", "RA000511", "RA000512", "RA000513", "RA000702", "RA000514", "RA000515",
    "RA000516", "RA000517", "RA000518", "RA000684", "RA000520", "RA000521", "RA000522", "RA000523",
    "RA000524", "RA000669", "RA000525", "RA000526", "RA000527", "RA000528", "RA000670", "RA000671",
    "RA000672", "RA000673", "RA000674", "RA000675", "RA000676", "RA000677", "RA000706", "RA000529",
    "RA000530", "RA000531", "RA000532", "RA000728", "RA000533", "RA000534", "RA000535", "RA000536",
    "RA000537", "RA000538", "RA000539", "RA000716", "RA000540", "RA000541", "RA000542", "RA000543",
    "RA000544", "RA000545", "RA000546", "RA000547", "RA000735", "RA000548", "RA000549", "RA000550",
    "RA000678", "RA000551", "RA000552", "RA000553", "RA000554", "RA000555", "RA000556", "RA000557",
    "RA000558", "RA000559", "RA000560", "RA000561", "RA000562", "RA000563", "RA000564", "RA000565",
    "RA000566", "RA000662", "RA000567", "RA000568", "RA000569", "RA000570", "RA000571", "RA000572",
    "RA000573", "RA000574", "RA000660", "RA000715", "RA000575", "RA000576", "RA000577", "RA000753",
    "RA000578", "RA000579", "RA000580", "RA000581", "RA000582", "RA000583", "RA000584", "RA000752",
    "RA000585", "RA000589", "RA000726", "RA000586", "RA000701", "RA000587", "RA000590", "RA000588",
    "RA000591", "RA000592", "RA000593", "RA000595", "RA000594", "RA000597", "RA000596", "RA000598",
    "RA000599", "RA000600", "RA000602", "RA000601", "RA000603", "RA000604", "RA000605", "RA000607",
    "RA000608", "RA000609", "RA000606", "RA000610", "RA000611", "RA000612", "RA000615", "RA000614",
    "RA000613", "RA000616", "RA000617", "RA000619", "RA000618", "RA000620", "RA000623", "RA000627",
    "RA000624", "RA000625", "RA000626", "RA000628", "RA000747", "RA000621", "RA000622", "RA000629",
    "RA000630", "RA000631", "RA000632", "RA000633", "RA000634", "RA000635", "RA000636", "RA000637",
    "RA000751", "RA000665", "RA000744", "RA000745", "RA000746", "RA000748", "RA000638", "RA000640",
    "RA000639", "RA000641", "RA000643", "RA000642", "RA000644", "RA000645", "RA000690", "RA000646",
    "RA000647", "RA000648", "RA000649", "RA000189", "RA000192", "RA000650", "RA000651", "RA000652",
    "RA000653",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholders() {
        assert!(is("RA888888"));
        assert!(is("RA999999"));
    }

    #[test]
    fn test_is() {
        assert!(is("RA000001"));
        assert!(!is("RA100001"));
    }
}
