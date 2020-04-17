use prost_types::{FileDescriptorSet, DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, EnumValueDescriptorProto};
use prost::encoding;
use std::collections::HashMap;
use prost::encoding::{decode_key, WireType};
use bytes::{Buf};

type Object = HashMap<String, Box<Value>>;

enum Value {
    Message(Object),
    Enum(String),
    // Map(),
    // Repeated(Value),
    Bool(bool),
    Str(String),
    // Bytes(Vec<u8>),
    // Int32(i64),
    // Int64(String),
    // Double(f64),
    // Any(String,Value),
    // Timestamp(String),
    // Duration(String),
}

enum Error {

}

struct MessageDescriptor {
    proto: DescriptorProto,
    fields: HashMap<u32, FieldDescriptorProto>,
}

struct EnumDescriptor {
    proto: EnumDescriptorProto,
    values: HashMap<i32, EnumValueDescriptorProto>,
}

struct FileSetDescriptor {
    desc: FileDescriptorSet,
    messages: HashMap<String, MessageDescriptor>,
    enums: HashMap<String, EnumDescriptor>,
}

struct Context {
    file_context: FileSetDescriptor,
    target: String
}

fn parse_message(fs: &FileSetDescriptor, target: MessageDescriptor, buf: &mut dyn Buf) -> Result<Object, Error> {
    let obj = Object::new();
    while buf.has_remaining() {
        let res = decode_key(buf)?;
        let field = target.fields.get(&res.0)?;
        // match res.1 {
        //     WireType::Varint => {},
        //     WireType::SixtyFourBit => {},
        //     WireType::LengthDelimited => {},
        //     WireType::StartGroup => {},
        //     WireType::EndGroup => {},
        //     WireType::ThirtyTwoBit => {},
        // }
    }
    Ok(obj)
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
