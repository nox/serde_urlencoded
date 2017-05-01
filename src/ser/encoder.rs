use url::form_urlencoded::Serializer;

///abstract layer for encoder.
pub trait UrlEncoder{
    /// add name/value pair
    fn append_pair(&mut self , name : &str , value : &str);

    fn finish(self) -> String;
}


pub struct FormUrlEncoder {
    url_encoder: Box<Serializer<String>>,
}

pub struct CachedEncoder {
    cache : Vec<(String , String)>
}

impl FormUrlEncoder {

    pub fn new() -> Self {
        FormUrlEncoder {url_encoder: Box::new(Serializer::new(String::new())) }
    }
}

impl UrlEncoder for FormUrlEncoder {

    fn append_pair(&mut self , name : &str , value : &str){
        (*self.url_encoder).append_pair(name , value);
    }

    fn finish(mut self) -> String {
        (*self.url_encoder).finish()
    }
}

impl CachedEncoder {

    pub fn new() -> CachedEncoder{
        CachedEncoder{cache : Vec::new()}
    }

    pub fn sort_by_name(&mut self) {
        self.cache.sort_by(|item_1 , item_2| item_1.0.cmp(&item_2.0));
    }
}

impl UrlEncoder for CachedEncoder {

    fn append_pair(&mut self , name : &str , value : &str) {
        self.cache.push((name.to_string() , value.to_string()));
    }

    fn finish(self) -> String {
        let mut serializer = Serializer::new(String::new());
        for (name , value) in self.cache.into_iter() {
            serializer.append_pair(&name , &value);
        }
        serializer.finish()
    }
}