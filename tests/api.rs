use speculum::data_model::Protocols;

#[test]
fn test_protocols()
{
    let p1 = Protocols {http: false, https: true, rsync: false};
    let p2 = Protocols {http: true, https: false, rsync: false};

    assert!(p1.eq(&p2) == false);
}
