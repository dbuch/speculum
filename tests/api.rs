use speculum::{Mirrors, Protocols};
use anyhow::Result;

static JSON_STRING: &str = r#"
        {
          "cutoff": 86400,
          "last_check": "2019-12-27T19:44:25.315Z",
          "num_checks": 95,
          "check_frequency": 873,
          "urls": [
            {
              "url": "https://mirror.aarnet.edu.au/pub/archlinux/",
              "protocol": "https",
              "last_sync": "2019-12-27T16:59:43Z",
              "completion_pct": 0.9894736842105263,
              "delay": 44304,
              "duration_avg": 1.0432762389487409,
              "duration_stddev": 0.510572728253044,
              "score": null,
              "active": true,
              "country": "Australia",
              "country_code": "AU",
              "isos": true,
              "ipv4": true,
              "ipv6": true,
              "details": "https://www.archlinux.org/mirrors/aarnet.edu.au/5/"
            },
            {
              "url": "https://mirror.aarnet.edu.au/pub/archlinux/",
              "protocol": "https",
              "last_sync": "2019-12-27T16:59:43Z",
              "completion_pct": 0.9894736842105263,
              "delay": 44304,
              "duration_avg": 1.0432762389487409,
              "duration_stddev": 0.510572728253044,
              "score": 0.1,
              "active": true,
              "country": "Australia",
              "country_code": "AU",
              "isos": true,
              "ipv4": true,
              "ipv6": true,
              "details": "https://www.archlinux.org/mirrors/aarnet.edu.au/5/"
            },
            {
              "url": "rsync://mirror.aarnet.edu.au/archlinux/",
              "protocol": "rsync",
              "last_sync": "2019-12-27T16:59:43Z",
              "completion_pct": 0.9894736842105263,
              "delay": 44302,
              "duration_avg": 3.418686384903757,
              "duration_stddev": 1.9958691735361513,
              "score": 17.909184400078157,
              "active": true,
              "country": "Australia",
              "country_code": "AU",
              "isos": true,
              "ipv4": true,
              "ipv6": true,
              "details": "https://www.archlinux.org/mirrors/aarnet.edu.au/6/"
            },
            {
              "url": "http://mir.archlinux.fr/",
              "protocol": "http",
              "last_sync": "2019-12-27T18:31:34Z",
              "completion_pct": 0.9789473684210527,
              "delay": 7428,
              "duration_avg": 0.3655665074625323,
              "duration_stddev": 0.3754786218161442,
              "score": 2.86468767686173,
              "active": true,
              "country": "France",
              "country_code": "FR",
              "isos": true,
              "ipv4": true,
              "ipv6": true,
              "details": "https://www.archlinux.org/mirrors/mir.archlinux.fr/16/"
            }
          ],
          "version": 3
        }
    "#;

#[test]
fn test_protocols() {
    let p1 = Protocols::from("http");
    let p2 = Protocols::from("https");
    let p3 = Protocols::from("http,https");
    let p4 = Protocols::from("rsync");
    assert!(p1.intercects(p2) == false);
    assert!(p1.intercects(p3) == true);
    assert!(p2.intercects(p3) == true);
    assert!(p3.intercects(p1) == true);
    assert!(p3.intercects(p2) == true);
    assert!(p4.intercects(p3) == false);
    assert!(p3.intercects(p3) == true);
}

#[test]
fn api() {
    let mut mirrors = Mirrors::load_from_utf8(JSON_STRING).unwrap();
    mirrors
        .filter_protocols(Protocols::from("https"))
        .order_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    assert_eq!(mirrors.len(), 1);
}

#[tokio::test]
async fn test_mirror_write() -> Result<()> {
    let mut mirrors = Mirrors::load_from_utf8(JSON_STRING).unwrap();
    let mut stdout = tokio::io::stdout();

    for mirror in mirrors.get_urls()
    {
        mirror.write(&mut stdout).await?;
    }
    Ok(())
}
