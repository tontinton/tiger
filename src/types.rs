#[derive(Clone)]
pub enum Type {
    Number(u8 /* size */, bool /* signed */),
}
