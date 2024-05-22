// use super::*;
//
// use std::fmt;
//
// pub struct WorkingStateDebug<'a>(pub &'a Task);
//
// impl fmt::Debug for WorkingStateDebug<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let task = self.0;
//
//         f.debug_struct("WorkingState")
//             .field("kind", &format_args!("{:?}", task.kind))
//             .finish()
//     }
// }
//
