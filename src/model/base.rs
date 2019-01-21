use serde_json::{Value};

#[allow(dead_code)]
pub struct BidRequest {
    id: String,
    imp: Vec<Imp>,
    site: Site,
    app: App,
    device: Device,
    user: User,
    test: i8,
    at: i8,
    tmax: i32,
    wseat: Vec<String>,
    allimps: i8,
    cur: Vec<String>,
    bcat: Vec<String>,
    badv: Vec<String>,
    regs: Regs,
}

#[allow(dead_code)]
pub struct Imp {
    id: String,
    banner: Banner,
    video: Video,
    native: Native,
    displaymanager: String,
    displaymanagerver: String,
    instl: i8,
    tagid: String,
    bidfloor: f32,
    bidfloorcur: String,
    secure: i8,
    iframebuster: Vec<String>,
    pmp: Pmp,
}

#[allow(dead_code)]
pub struct Banner {
    w: i32,
    h: i32,
    wmax: i32,
    hmax: i32,
    wmin: i32,
    hmin: i32,
    id: String,
    btype: Vec<i32>,
    battr: Vec<i32>,
    pos: i32,
    mimes: Vec<String>,
    topframe: i8,
    expdir: Vec<i32>,
    api: Vec<i32>,
    ext: Value,
}

#[allow(dead_code)]
pub struct Video {
    mimes: Vec<String>,
    minduration: i32,
    maxduration: i32,
    protocol: i32,
    protocols: Vec<i32>,
    w: i32,
    h: i32,
    startdelay: i32,
    linearity: i32,
    sequence: i32,
    battr: Vec<i32>,
    maxextended: u8,
    minbitrate: i32,
    maxbitrate: i32,
    boxingallowed: i8,
    playbackmethod: Vec<i32>,
    delivery: Vec<i32>,
    pos: i32,
    companion: Vec<Banner>,
    api: Vec<i32>,
    companiontype: Vec<i32>,
    ext: Value,
}

#[allow(dead_code)]
pub struct Native {
    request: String,
    ver: String,
    api: Vec<i32>,
    battr: Vec<i32>,
    ext: Value,
}

#[allow(dead_code)]
pub struct Site {
    id: String,
    name: String,
    domain: String,
    cat: Vec<String>,
    sectioncat: Vec<String>,
    pagecat: Vec<String>,
    page: String,
    // real name : ref
    ref_: String,
    search: String,
    mobile: i8,
    privacypolicy: i32,
    publisher: Publisher,
    content: Content,
    keywords: String,
    ext: Value,
}

#[allow(dead_code)]
pub struct App {
    id: String,
    name: String,
    bundle: String,
    domain: String,
    storeurl: String,
    cat: Vec<String>,
    sectioncat: Vec<String>,
    pagecat: Vec<String>,
    ver: String,
    privacypolicy: i8,
    paid: i8,
    publisher: Publisher,
    content: Content,
    keywords: String,
    ext: Value,
}

#[allow(dead_code)]
pub struct Publisher {
    id: String,
    name: String,
    cat: Vec<String>,
    domain: String,
    ext: Value,
}

#[allow(dead_code)]
pub struct Content {
    id: String,
    episode: i32,
    title: String,
    series: String,
    season: String,
    producer: Producer,
    url: String,
    cat: Vec<String>,
    videoquality: i32,
    context: i32,
    contentrating: String,
    userrating: String,
    qagmediarating: i32,
    keywords: String,
    livestream: i8,
    sourcerelationship: i8,
    len: i32,
    language: String,
    embeddable: i8,
    ext: Value,
}

#[allow(dead_code)]
pub struct Producer {
    id: String,
    name: String,
    cat: Vec<String>,
    domain: String,
    ext: Value,
}

#[allow(dead_code)]
pub struct Device {
    ua: String,
    geo: Geo,
    dnt: i8,
    lmt: i8,
    ip: String,
    ipv6: String,
    devicetype: i32,
    make: String,
    model: String,
    os: String,
    osv: String,
    hwv: String,
    h: i32,
    w: i32,
    ppi: i32,
    pxratio: f32,
    js: i8,
    flashver: String,
    language: String,
    carrier: String,
    connectiontype: i32,
    ifa: String,
    didsha1: String,
    didm5: String,
    dpidsha1: String,
    dpidmd5: String,
    macsha1: String,
    macmd5: String,
    ext: Value,
}


#[allow(dead_code)]
pub struct Geo {
    lat: f32,
    lon: f32,
    // Field name : type
    type_: i32,
    country: String,
    region: String,
    regionfips104: String,
    metro: String,
    city: String,
    zip: String,
    utcoffset: i32,
    ext: Value,
}

#[allow(dead_code)]
pub struct User {
    id: String,
    buyeruid: String,
    yob: i32,
    gender: String,
    keywords: String,
    customdata: String,
    geo: Geo,
    data: Vec<Data>,
    ext: Value,
}

#[allow(dead_code)]
pub struct Data {
    id: String,
    name: String,
    segment: Vec<Segment>,
    ext: Value,
}


#[allow(dead_code)]
pub struct Segment {
    id: String,
    name: String,
    value: String,
    ext: Value,
}

#[allow(dead_code)]
pub struct Regs {
    coppa: i8,
    ext: Value,
}

#[allow(dead_code)]
pub struct Pmp {
    private_auction: i8,
    deals: Vec<Deal>,
    ext: Value,
}

#[allow(dead_code)]
pub struct Deal {
    id: String,
    bidfloor: f32,
    bidfloorcur: String,
    at: i8,
    wseat: Vec<i32>,
    wadomain: Vec<i32>,
    ext: Value,
}

