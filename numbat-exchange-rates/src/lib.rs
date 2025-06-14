use std::collections::HashMap;

#[cfg(feature = "fetch-exchangerates")]
use quick_xml::events::Event;
#[cfg(feature = "fetch-exchangerates")]
use quick_xml::reader::Reader;

pub type ExchangeRates = HashMap<String, f64>;

#[cfg(feature = "fetch-exchangerates")]
pub fn parse_exchange_rates(xml_content: &str) -> Option<ExchangeRates> {
    let mut rates = ExchangeRates::default();

    let mut reader = Reader::from_str(xml_content);
    loop {
        match reader.read_event().ok()? {
            Event::Eof => break,
            Event::Empty(e) => {
                if e.local_name().as_ref() != b"Cube" {
                    continue;
                }
                let currency = &e
                    .try_get_attribute("currency")
                    .ok()??
                    .unescape_value()
                    .ok()?;
                let rate = &e.try_get_attribute("rate").ok()??.unescape_value().ok()?;
                let rate = rate.parse().ok()?;

                rates.insert(currency.to_string(), rate);
            }
            _ => {}
        }
    }

    Some(rates)
}

#[cfg(feature = "fetch-exchangerates")]
const ECB_XML_URL: &str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

#[cfg(feature = "fetch-exchangerates")]
fn fetch_ecb_xml() -> Option<String> {
    attohttpc::get(ECB_XML_URL).send().ok()?.text().ok()
}

#[cfg(feature = "fetch-exchangerates")]
pub fn fetch_exchange_rates() -> Option<ExchangeRates> {
    let xml_content = fetch_ecb_xml()?;
    parse_exchange_rates(&xml_content)
}

#[cfg(test)]
#[cfg(feature = "fetch-exchangerates")]
mod tests {
    use super::*;

    #[test]
    fn fetch_exchange_rates_works() {
        fetch_exchange_rates();
    }
}
