extern crate reqwest;
use reqwest::header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert(
        "accept-language",
        "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7,zh-TW;q=0.6"
            .parse()
            .unwrap(),
    );
    headers.insert(header::COOKIE, "buvid3=B96D1196-DD0B-F462-93DD-9ADED881B6CD00836infoc; b_nut=1712455200; i-wanna-go-back=-1; b_ut=7; _uuid=DD12E1057-54B8-51910-CCCC-16F639F2972700950infoc; enable_web_push=DISABLE; buvid4=08C663F6-1206-BEA4-BAEF-AC717D7181D501509-024040702-nbcKV067BWgit4396QMiMg%3D%3D; DedeUserID=88515203; DedeUserID__ckMd5=471946cded06472d; rpdid=|(k|JukYJYJR0J'u~ukR~~k)l; FEED_LIVE_VERSION=V_WATCHLATER_PIP_WINDOW3; header_theme_version=CLOSE; buvid_fp_plain=undefined; is-2022-channel=1; CURRENT_BLACKGAP=0; LIVE_BUVID=AUTO4817188555637683; CURRENT_QUALITY=120; PVID=1; home_feed_column=5; browser_resolution=1920-911; hit-dyn-v2=1; fingerprint=7aa10d759fb0c7a0096b71d2b98215f3; buvid_fp=7aa10d759fb0c7a0096b71d2b98215f3; bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MjEwMjI1MTcsImlhdCI6MTcyMDc2MzI1NywicGx0IjotMX0.fm2teEuh1g3KEVAvf4AujpgwuDb6xdcc0lQm2iiEvnI; bili_ticket_expires=1721022457; SESSDATA=f276eaad%2C1736315320%2Caff5c%2A71CjD9Xx6g-hHeYHBH-iuOx7c2YtHgGSH_BX7ZhpUO2S3lfCLtOR9tw7xjQ9dRxQR9N4gSVmNzSzVSTnlDS0trN2loYzNxWXhMVkdpSlp4V2JuUGNPZWFQdFl3eVZqR183bkYyQk53dXk1VkVkY242dDZYT3d3NU9SMUtER2lFenAzUUpTX0V3eGl3IIEC; bili_jct=1c3b2abb4697dbc117421bc0235bf638; CURRENT_FNVAL=4048; bp_t_offset_88515203=953530808985976832; b_lsid=F5FFD210A_190ACD305C8; bsource=search_baidu; sid=65a5qcdf".parse().unwrap());
    headers.insert("origin", "https://search.bilibili.com".parse().unwrap());
    headers.insert("priority", "u=1, i".parse().unwrap());
    headers.insert("referer", "https://search.bilibili.com/article?vt=89171350&keyword=rust&from_source=article&order=pubdate".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\", \"Google Chrome\";v=\"126\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get("https://api.bilibili.com/x/web-interface/wbi/search/type?category_id=0&search_type=article&ad_resource=5646&__refresh__=true&_extra=&context=&page=1&page_size=20&order=pubdate&duration=&from_source=&from_spmid=333.337&platform=pc&highlight=1&single_column=0&keyword=rust&qv_id=NLpBCaOsniDnNpHvpbaa9I0Je0wy7lp4&source_tag=3&gaia_vtoken=&web_location=1430654&w_rid=15117c3cb590b480678d36e8b50286ee&wts=1720889194")
        .headers(headers)
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
