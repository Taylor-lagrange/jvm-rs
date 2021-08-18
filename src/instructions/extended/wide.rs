// use crate::instructions::base::branch::*;
// use crate::instructions::base::bytecode_reader::*;
// use crate::instructions::base::instruction::*;
// use crate::runtime::thread::*;

// 加载类指令、存储类指令、ret指令和iinc指
// 令需要按索引访问局部变量表，索引以uint8的形
// 式存在字节码中。对于⼤部分⽅法来说，局部变
// 量表⼤⼩都不会超过256，所以⽤⼀字节来表⽰索
// 引就够了。但是如果有⽅法的局部变量表超过这
// 限制呢？Java虚拟机规范定义了wide指令来扩展
// 前述指令。

// pub struct WIDE {}

// impl Instruction for WIDE {
//   fn execute(&mut self, reader: &mut BytecodeReader, frame: &mut Frame) {
//     let opcode = reader.read_u8();
//   }
// }

// Extend local variable index by additional bytes
// type WIDE struct {
// 	modifiedInstruction base.Instruction
// }

// func (self *WIDE) FetchOperands(reader *base.BytecodeReader) {
// 	opcode := reader.ReadUint8()
// 	switch opcode {
// 	case 0x15:
// 		inst := &loads.ILOAD{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x16:
// 		inst := &loads.LLOAD{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x17:
// 		inst := &loads.FLOAD{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x18:
// 		inst := &loads.DLOAD{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x19:
// 		inst := &loads.ALOAD{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x36:
// 		inst := &stores.ISTORE{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x37:
// 		inst := &stores.LSTORE{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x38:
// 		inst := &stores.FSTORE{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x39:
// 		inst := &stores.DSTORE{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x3a:
// 		inst := &stores.ASTORE{}
// 		inst.Index = uint(reader.ReadUint16())
// 		self.modifiedInstruction = inst
// 	case 0x84:
// 		inst := &math.IINC{}
// 		inst.Index = uint(reader.ReadUint16())
// 		inst.Const = int32(reader.ReadInt16())
// 		self.modifiedInstruction = inst
// 	case 0xa9: // ret
// 		panic("Unsupported opcode: 0xa9!")
// 	}
// }

// func (self *WIDE) Execute(frame *rtda.Frame) {
// 	self.modifiedInstruction.Execute(frame)
// }
