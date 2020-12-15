extern crate xml;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use xml::reader::{EventReader, XmlEvent};

pub fn parse_msg(msg: &str) -> Option<HashMap<String, String>> {
    if msg.is_empty() {
        return None;
    }

    let parser = EventReader::from_str(msg);
    let mut result: HashMap<String, String> = HashMap::new();

    let mut tag_names: Vec<String> = vec![];
    for elem in parser {
        match elem {
            Ok(XmlEvent::StartElement { name, .. }) => {
                tag_names.push(name.local_name);
            }
            Ok(XmlEvent::CData(data)) => {
                let tag_name = tag_names.pop();
                result.insert(tag_name.unwrap(), data);
            }

            Ok(XmlEvent::Characters(s)) => {
                let tag_name = tag_names.pop();
                result.insert(tag_name.unwrap(), s);
            }
            Err(e) => {
                return None;
            }
            _ => {}
        }
    }

    Some(result)
}

pub fn build_text_msg(from: &String, to: &String, content: String) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp: String = format!("{}", since_the_epoch.as_millis());
    format!(
        "<xml>
    <ToUserName>{}</ToUserName>
    <FromUserName>{}</FromUserName>
    <MsgType>text</MsgType>
    <Content>{}</Content>
    <CreateTime>{}</CreateTime>
    </xml>",
        to, from, content, timestamp
    )
}

#[cfg(test)]
mod parse_xml_msg_tests {
    use super::*;

    #[test]
    fn test_parse_msg() {
        let msg = "<xml><ToUserName><![CDATA[gh_c61fe67016df]]></ToUserName>\n<FromUserName><![CDATA[oZVaUxFWzZtiZowApVrlQzT7CRjw]]></FromUserName>\n<CreateTime>1606870657</CreateTime>\n<MsgType><![CDATA[text]]></MsgType>\n<Content><![CDATA[哈哈]]></Content>\n<MsgId>23004855674080303</MsgId>\n</xml>";
        let map = parse_msg(msg);
        println!("map: {:?}", map);
    }
}
