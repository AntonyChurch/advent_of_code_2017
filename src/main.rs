extern crate adventofcode;

fn main(){
    println!("Answer 1a: {}", adventofcode::one_a(String::from("77736991856689225253142335214746294932318813454849177823468674346512426482777696993348135287531487622845155339235443718798255411492778415157351753377959586612882455464736285648473397681163729345143319577258292849619491486748832944425643737899293811819448271546283914592546989275992844383947572926628695617661344293284789225493932487897149244685921644561896799491668147588536732985476538413354195246785378443492137893161362862587297219368699689318441563683292683855151652394244688119527728613756153348584975372656877565662527436152551476175644428333449297581939357656843784849965764796365272113837436618857363585783813291999774718355479485961244782148994281845717611589612672436243788252212252489833952785291284935439662751339273847424621193587955284885915987692812313251556836958571335334281322495251889724281863765636441971178795365413267178792118544937392522893132283573129821178591214594778712292228515169348771198167462495988252456944269678515277886142827218825358561772588377998394984947946121983115158951297156321289231481348126998584455974277123213413359859659339792627742476688827577318285573236187838749444212666293172899385531383551142896847178342163129883523694183388123567744916752899386265368245342587281521723872555392212596227684414269667696229995976182762587281829533181925696289733325513618571116199419759821597197636415243789757789129824537812428338192536462468554399548893532588928486825398895911533744671691387494516395641555683144968644717265849634943691721391779987198764147667349266877149238695714118982841721323853294642175381514347345237721288281254828745122878268792661867994785585131534136646954347165597315643658739688567246339618795777125767432162928257331951255792438831957359141651634491912746875748363394329848227391812251812842263277229514125426682179711184717737714178235995431465217547759282779499842892993556918977773236196185348965713241211365895519697294982523166196268941976859987925578945185217127344619169353395993198368185217391883839449331638641744279836858188235296951745922667612379649453277174224722894599153367373494255388826855322712652812127873536473277")));
    println!("Ansert 1b: {}", adventofcode::one_b(String::from("77736991856689225253142335214746294932318813454849177823468674346512426482777696993348135287531487622845155339235443718798255411492778415157351753377959586612882455464736285648473397681163729345143319577258292849619491486748832944425643737899293811819448271546283914592546989275992844383947572926628695617661344293284789225493932487897149244685921644561896799491668147588536732985476538413354195246785378443492137893161362862587297219368699689318441563683292683855151652394244688119527728613756153348584975372656877565662527436152551476175644428333449297581939357656843784849965764796365272113837436618857363585783813291999774718355479485961244782148994281845717611589612672436243788252212252489833952785291284935439662751339273847424621193587955284885915987692812313251556836958571335334281322495251889724281863765636441971178795365413267178792118544937392522893132283573129821178591214594778712292228515169348771198167462495988252456944269678515277886142827218825358561772588377998394984947946121983115158951297156321289231481348126998584455974277123213413359859659339792627742476688827577318285573236187838749444212666293172899385531383551142896847178342163129883523694183388123567744916752899386265368245342587281521723872555392212596227684414269667696229995976182762587281829533181925696289733325513618571116199419759821597197636415243789757789129824537812428338192536462468554399548893532588928486825398895911533744671691387494516395641555683144968644717265849634943691721391779987198764147667349266877149238695714118982841721323853294642175381514347345237721288281254828745122878268792661867994785585131534136646954347165597315643658739688567246339618795777125767432162928257331951255792438831957359141651634491912746875748363394329848227391812251812842263277229514125426682179711184717737714178235995431465217547759282779499842892993556918977773236196185348965713241211365895519697294982523166196268941976859987925578945185217127344619169353395993198368185217391883839449331638641744279836858188235296951745922667612379649453277174224722894599153367373494255388826855322712652812127873536473277")));

    twoa();
}

fn twoa(){
    let mut whole: Vec<Vec<u32>> = Vec::new();

    let row1: Vec<u32> = vec![278,	1689,	250,	1512,	1792,	1974,	175,	1639,	235,	1635,	1690,	1947,	810,	224,	928,	859];
    let row2: Vec<u32> = vec![160,	50,	55,	81,	68,	130,	145,	21,	211,	136,	119,	78,	174,	155,	149,	72];
    let row3: Vec<u32> = vec![4284,	185,	4499,	273,	4750,	4620,	4779,	4669,	2333,	231,	416,	1603,	197,	922,	5149,	2993];
    let row4: Vec<u32> = vec![120,	124,	104,	1015,	1467,	110,	299,	320,	1516,	137,	1473,	132,	1229,	1329,	1430,	392];
    let row5: Vec<u32> = vec![257,	234,	3409,	2914,	2993,	3291,	368,	284,	259,	3445,	245,	1400,	3276,	339,	2207,	233];
    let row6: Vec<u32> = vec![1259,	78,	811,	99,	2295,	1628,	3264,	2616,	116,	3069,	2622,	1696,	1457,	1532,	268,	82];
    let row7: Vec<u32> = vec![868,	619,	139,	522,	168,	872,	176,	160,	1010,	200,	974,	1008,	1139,	552,	510,	1083];
    let row8: Vec<u32> = vec![1982,	224,	3003,	234,	212,	1293,	1453,	3359,	326,	3627,	3276,	3347,	1438,	2910,	248,	2512];
    let row9: Vec<u32> = vec![4964,	527,	5108,	4742,	4282,	4561,	4070,	3540,	196,	228,	3639,	4848,	152,	1174,	5005,	202];
    let row10: Vec<u32> = vec![1381,	1480,	116,	435,	980,	1022,	155,	1452,	1372,	121,	128,	869,	1043,	826,	1398,	137];
    let row11: Vec<u32> = vec![2067,	2153,	622,	1479,	2405,	1134,	2160,	1057,	819,	99,	106,	1628,	1538,	108,	112,	1732];
    let row12: Vec<u32> = vec![4535,	2729,	4960,	241,	4372,	3960,	248,	267,	230,	5083,	827,	1843,	3488,	4762,	2294,	3932];
    let row13: Vec<u32> = vec![3245,	190,	2249,	2812,	2620,	2743,	2209,	465,	139,	2757,	203,	2832,	2454,	177,	2799,	2278];
    let row14: Vec<u32> = vec![1308,	797,	498,	791,	1312,	99,	1402,	1332,	521,	1354,	1339,	101,	367,	1333,	111,	92];
    let row15: Vec<u32> = vec![149,	4140,	112,	3748,	148,	815,	4261,	138,	1422,	2670,	32,	334,	2029,	4750,	4472,	2010];
    let row16: Vec<u32> = vec![114,	605,	94,	136,	96,	167,	553,	395,	164,	159,	284,	104,	530,	551,	544,	18];
    
    whole.push(row1);
    whole.push(row2);
    whole.push(row3);
    whole.push(row4);
    whole.push(row5);
    whole.push(row6);
    whole.push(row7);
    whole.push(row8);
    whole.push(row9);
    whole.push(row10);
    whole.push(row11);
    whole.push(row12);
    whole.push(row13);
    whole.push(row14);
    whole.push(row15);
    whole.push(row16);

    let val = adventofcode::two_a(whole);

    println!("Answer 2a: {}", val);
}