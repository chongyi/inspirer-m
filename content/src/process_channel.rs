//! 处理通道，即实际处理内容数据流的对象
//!
//! 我们在提交或读取内容时，以往的逻辑是通过读写接口进行定义和执行的，
//! 作为一个处理内核，考虑到扩展性以及适用性，我们将原始的读写接口抽象为数据处理通道，
//! 这不同于传统的接口，通道不关心数据对象的具体类型和结构，也不会做细致的数据验证，
//! 凡是符合该通道要求的数据都会进行处理，一视同仁。
//!
//! 扩展通道类型既是扩展整个内容管理系统的底层逻辑。基于这个逻辑之上，可以进行上层封装，
//! 例如投递至数据流处理通道前的数据校验。
//!
//! 数据处理通道是一个抽象的对象，其内部可能包含很多的共享数据、扩展插件，这取决于通道本身
//! 的设定。例如常见的通道一定会包含一个上下文对象，里面存储包括当前会话的用户信息或者
//! 权限信息等等。
//!

mod read;
mod write;

pub use write::{WriteProcessChannel};