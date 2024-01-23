use crate::flag::Flag;
use game::Item;
use {
    crate::{
        files::{msgbn::MsgBn, FromFile},
        Error, Result,
    },
    bytey::*,
};

type Ref<'input> = core::cell::Ref<'input, [u8]>;
type RefSized<'input, const N: usize> = core::cell::Ref<'input, [u8; N]>;
type RefMut<'input> = &'input mut [u8];

type Next = Option<u16>;

#[derive(Debug)]
pub struct Flow<'input> {
    #[allow(unused)]
    header: List<Ref<'input>, HEADER_LEN>,
    steps: List<Ref<'input>, STEP_LEN>,
    branches: Branches<'input>,
}

impl<'input> Flow<'input> {
    pub fn steps<'s>(&'s self) -> Steps<'s, 'input> {
        Steps(self)
    }
}

impl<'input> FromFile for Flow<'input> {
    type PathArgs = str;
    type Input = Ref<'input>;

    fn path(args: &Self::PathArgs) -> String {
        format!("World/Flow/{}.msbf", args)
    }

    fn from_file(input: Self::Input) -> Result<Self>
    where
        Self: Sized,
    {
        // Line 1: Should contain magic "MsgFlwBn"
        let msgbn = MsgBn::<Ref<'input>, 2>::try_read(input, MSGFLWBN)?;

        // Line 2: seems to refer to length of the file

        // Line 3: Should contain magic "FLW3"
        let flw = msgbn.get(FLW3).ok_or_else(|| Error::new("No FLW3"))?;

        // Line 4: header, which contains the step and branch count
        let (step_ct, branch_ct, index) = get_flw(&flw)?;

        let (header, body) = Ref::map_split(flw, |flw| flw.split_at(0x10));
        let (steps, branches) = Ref::map_split(body, |body| body.split_at(index));

        // Branches are 2 bytes each, and appear sequentially on the first line following the last step
        // If the number of branches is not a multiple of 8 (i.e. it doesn't fill up a line), that line
        // will be padded with "AB"

        let header = List::<Ref, HEADER_LEN>::new(1, header).ok_or_else(|| Error::new("malformed header"))?;
        let steps = List::<Ref, STEP_LEN>::new(step_ct, steps).ok_or_else(|| Error::new("unimpl33"))?;
        let branches =
            Branches(List::<Ref, BRANCH_LEN>::new(branch_ct, branches).ok_or_else(|| Error::new("unimpl33"))?);

        // Following the branches, the magic "FEN1" appears, followed by two bytes (size of remaining garbage?)

        Ok(Self { header, steps, branches })
    }
}

typedef! {
    struct Header: FromBytes<'_> [0x10] {
        [0] steps: u16,
        [2] branches: u16,
    }
}

#[derive(Debug)]
pub struct Steps<'flow, 'input>(&'flow Flow<'input>);

impl<'flow, 'input> Steps<'flow, 'input> {
    pub fn get(&self, index: u16) -> Option<Result<Step>> {
        self.0.steps.get(index).map(|bytes| Step::from_bytes(&bytes, &self.0.branches))
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<Step>> + '_ {
        self.0.steps.iter().map(move |bytes| Step::from_bytes(&bytes, &self.0.branches))
    }
}

#[derive(Debug)]
pub enum Step {
    Text { next: Next },
    Branch { command: Branch, branches: Vec<Next> },
    Action { command: Action, next: Next },
    Start { next: Next },
    Goto { next: Next },
}

impl Step {
    fn from_bytes(bytes: &[u8; STEP_LEN], branches: &Branches) -> Result<Self> {
        typedef! { struct Inner: FromBytes<'_> [STEP_LEN] {
            [0] kind: u8,
            [4] value: u32,
            [8] next: u16,
            [0xA] command: u16,
            [0xC] count: u16,
            [0xE] branch: u16,
        }}
        let step = Inner::from_bytes(bytes);
        let next = next(step.next);
        match step.kind {
            1 => Ok(Self::Text { next }),
            2 => Self::branch(branches, Branch::new(step.command, step.value), step.count, step.branch),
            3 => Ok(Self::Action { command: Action::new(step.command, step.value), next }),
            4 => Ok(Self::Start { next }),
            5 => Ok(Self::Goto { next }),
            kind => Err(Error::new(format!("Invalid flow value: {:X}.", kind))),
        }
    }

    fn branch(branches: &Branches, command: Branch, count: u16, branch: u16) -> Result<Self> {
        let branches = branches.iter().skip(branch as usize).take(count as usize).map(next).collect::<Vec<_>>();
        if branches.len() < count as usize {
            Err(Error::new(format!("Could not get {} branches.", count)))
        } else {
            Ok(Self::Branch { command, branches })
        }
    }
}

#[derive(Debug)]
pub enum Branch {
    Other(u16, u32),
}

impl Branch {
    fn new(kind: u16, value: u32) -> Self {
        Self::Other(kind, value)
    }
}

#[derive(Debug)]
pub enum Action {
    Other(u16, u32),
}

impl Action {
    fn new(kind: u16, value: u32) -> Self {
        Self::Other(kind, value)
    }
}

#[derive(Debug)]
struct Branches<'input>(List<Ref<'input>, BRANCH_LEN>);

impl<'input> Branches<'input> {
    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        self.0.iter().map(|bytes| u16::from_bytes(&*bytes))
    }
}

#[derive(Debug)]
pub struct FlowMut<'input> {
    #[allow(unused)]
    header: List<RefMut<'input>, HEADER_LEN>,
    steps: List<RefMut<'input>, STEP_LEN>,
    branches: BranchesMut<'input>,
}

impl<'input> FlowMut<'input> {
    pub fn get_mut<'s>(&'s mut self, index: u16) -> Option<StepMut<'s, 'input>> {
        if self.steps.get_mut(index).is_some() {
            Some(StepMut { flow: self, index })
        } else {
            None
        }
    }

    /// Hacky thing that prints out MSBF file info
    pub fn research(&self) {
        typedef! {
            struct Inner: FromBytes<'_> [STEP_LEN] {
            [0] kind: u8,
            [1] arg1: u8,
            [2] arg2: u8,
            [3] arg3: u8,
            [4] arg4: u16,
            [6] value: u16,
            [8] next: u16,
            [0xA] command: u16,
            [0xC] count: u16,
            [0xE] branch: u16,
            }
        }

        typedef! {
            struct InnerBranch: FromBytes<'_> [STEP_LEN] {
            [0] arg0: u8,
            [1] arg1: u8,
            }
        }

        println!("index,kind,arg1,arg2,arg3,arg4,value,next,command,count,branch,0,1,notes");
        let mut step: Inner;
        for i in 0..(self.steps.inner.len() / STEP_LEN) {
            step =
                unsafe { Inner::from_slice_unchecked(&self.steps.inner[(i * STEP_LEN)..((i * STEP_LEN) + STEP_LEN)]) };
            println!(
                "[{: >3}],{},{},{},{},{},{},{},{},{},{}",
                i,
                step.kind,
                step.arg1,
                step.arg2,
                step.arg3,
                step.arg4,
                step.value,
                step.next,
                step.command,
                step.count,
                step.branch
            );
        }
        println!("branches");
        let mut branch: InnerBranch;
        for i in 0..(&self.branches.inner.len() / 2) {
            branch = unsafe { InnerBranch::from_slice_unchecked(&self.branches.inner[(i * 2)..((i * 2) + 2)]) };
            println!("[{}],{},{}", i, branch.arg0, branch.arg1);
        }
    }

    /// Generates a dot graph to visualize the MSBF/MSBT files.
    /// Plug the results into: http://edotor.net/
    pub fn edotor(&self, labels: Vec<(String, String)>) {
        typedef! {
            struct Inner: FromBytes<'_> [STEP_LEN] {
            [0] kind: u8,
            [1] arg1: u8,
            [2] arg2: u8,
            [3] arg3: u8,
            [4] arg4: u16,
            [6] value: u16,
            [8] next: u16,
            [0xA] command: u16,
            [0xC] count: u16,
            [0xE] branch: u16,
            }
        }

        typedef! {
            struct InnerBranch: FromBytes<'_> [STEP_LEN] {
            [0] arg0: u16,
            }
        }

        // Find Branches first
        let mut branches: Vec<InnerBranch> = Vec::new();
        for i in 0..(&self.branches.inner.len() / 2) {
            branches.push(unsafe { InnerBranch::from_slice_unchecked(&self.branches.inner[(i * 2)..((i * 2) + 2)]) });
        }

        println!("digraph {{");
        for i in 0..(self.steps.inner.len() / STEP_LEN) {
            let step =
                unsafe { Inner::from_slice_unchecked(&self.steps.inner[(i * STEP_LEN)..((i * STEP_LEN) + STEP_LEN)]) };

            match step.kind {
                1 => {
                    let label = if let Some((label, msg)) = labels.get(step.count as usize) {
                        let mut bytes: Vec<u8> = msg.as_bytes().into();
                        bytes = bytes
                            .iter()
                            .map(|&b| if b != 0xA && (b < 0x20 || b > 0x7E) { 0xEF } else { b })
                            .collect::<Vec<_>>();

                        let msg = String::from_utf8_lossy(&*bytes);
                        Some((label, str::replace(&*msg, "\n", "\\n")))
                    } else {
                        None
                    };

                    println!(
                        "    {} [color=green4, shape=rect, label=\"{}\\n{}\"]",
                        i,
                        i,
                        if let Some((label, msg)) = label {
                            format!("\\\"{}\\\"\\n\\n\\\"{}\\\"", label, msg)
                        } else {
                            "".to_owned()
                        }
                    );
                    println!(
                        "    {} -> {}",
                        i,
                        if step.next == 65535 { format!("END_{}", i) } else { step.next.to_string() }
                    );
                },
                2 => {
                    let label = if step.arg1 == 6 && step.command == 10 {
                        format!("Check Event Flag {}", Flag::get_true_flag(step.arg4))
                    } else if step.arg1 == 0 && step.command == 6 {
                        format!("Got {} Rupees?", step.arg4)
                    } else if step.arg1 == 0 && step.command == 0xE {
                        format!("Check Course Flag {}", step.arg4)
                    } else if step.arg1 == 0 && step.command == 0xD {
                        format!("Check Local (2) Flag {}", step.arg4)
                    } else {
                        "???".to_owned()
                    };

                    println!("    {} [color=orange3, shape=diamond, label=\"{}\\n{}\"]", i, i, label);

                    for j in 0..step.count {
                        let branch = branches.get((step.branch + j) as usize).unwrap();
                        println!(
                            "    {} -> {}",
                            i,
                            if branch.arg0 == 65535 { format!("END_{}_{}", i, j) } else { branch.arg0.to_string() }
                        );
                    }
                },
                3 => {
                    let label = if step.arg1 == 6 && (step.command == 0 || step.command == 0xE) {
                        format!("Set Event Flag {}", Flag::get_true_flag(step.arg4))
                    } else if step.arg1 == 0 && step.command == 28 {
                        format!("Set Local (2) Flag {}", step.arg4)
                    } else if step.arg1 == 0 && step.command == 30 {
                        format!("Set Course Flag {}", step.arg4)
                    } else if step.arg1 == 6 && step.command == 11 {
                        Item::try_from(step.arg4).unwrap().as_str().to_owned()
                    } else {
                        "?".to_owned()
                    };

                    println!("    {} [label=\"{}\\n{}\"]", i, i, label);
                    println!(
                        "    {} -> {}",
                        i,
                        if step.next == 65535 { format!("END_{}", i) } else { step.next.to_string() }
                    );
                },
                4 => {
                    println!("    {} [color=cyan3, shape=doublecircle]", i);
                    println!(
                        "    {} -> {}",
                        i,
                        if step.next == 65535 { format!("END_{}", i) } else { step.next.to_string() }
                    );
                },
                5 => {
                    println!("    {} [color=purple, shape=pentagon, label=\"{}\\nGOTO {}\"]", i, i, step.next);
                    println!(
                        "    {} -> {}",
                        i,
                        if step.next == 65535 { format!("END_{}", i) } else { step.next.to_string() }
                    );
                },
                _ => unreachable!(),
            }
        }

        println!("}}");
    }
}

impl<'input> FromFile for FlowMut<'input> {
    type PathArgs = str;
    type Input = RefMut<'input>;

    fn path(args: &Self::PathArgs) -> String {
        format!("World/Flow/{}.msbf", args)
    }

    fn from_file(input: Self::Input) -> Result<Self>
    where
        Self: Sized,
    {
        let msgbn = MsgBn::<RefMut<'input>, 2>::try_read(input, MSGFLWBN)?;
        let flw = msgbn.into_section(FLW3).ok_or_else(|| Error::new("No FLW3"))?;
        let (step_ct, branch_ct, index) = get_flw(flw)?;

        let (header, body) = flw.split_at_mut(0x10);
        let (steps, branches) = body.split_at_mut(index);

        let header = List::<RefMut, HEADER_LEN>::new(1, header).ok_or_else(|| Error::new("unimpl33"))?;
        let steps = List::<RefMut, STEP_LEN>::new(step_ct, steps).ok_or_else(|| Error::new("unimpl33"))?;
        let branches = List::<RefMut, BRANCH_LEN>::new(branch_ct, branches).ok_or_else(|| Error::new("unimpl33"))?;
        Ok(Self { header, steps, branches })
    }
}

#[derive(Debug)]
pub struct StepMut<'flow, 'input> {
    flow: &'flow mut FlowMut<'input>,
    index: u16,
}

impl<'flow, 'input> StepMut<'flow, 'input> {
    pub fn into_text(self) -> Option<TextMut<'flow, 'input>> {
        if self.flow.steps.get_mut(self.index).unwrap()[0] == 1 {
            Some(TextMut(self))
        } else {
            None
        }
    }

    pub fn into_branch(self) -> Option<BranchMut<'flow, 'input>> {
        if self.flow.steps.get_mut(self.index).unwrap()[0] == 2 {
            Some(BranchMut(self))
        } else {
            None
        }
    }

    pub fn into_action(self) -> Option<ActionMut<'flow, 'input>> {
        if self.flow.steps.get_mut(self.index).unwrap()[0] == 3 {
            Some(ActionMut(self))
        } else {
            None
        }
    }

    pub fn into_start(self) -> Option<StartMut<'flow, 'input>> {
        if self.flow.steps.get_mut(self.index).unwrap()[0] == 4 {
            Some(StartMut(self))
        } else {
            None
        }
    }

    pub fn into_goto(self) -> Option<GotoMut<'flow, 'input>> {
        if self.flow.steps.get_mut(self.index).unwrap()[0] == 5 {
            Some(GotoMut(self))
        } else {
            None
        }
    }

    pub fn convert_into_action(self) -> Option<ActionMut<'flow, 'input>> {
        unsafe {
            *self.flow.steps.get_mut(self.index).unwrap().get_unchecked_mut(0) = 3;
            Some(ActionMut(self))
        }
    }

    pub fn convert_into_branch(mut self, count: u8, branch_index: u8) -> Option<BranchMut<'flow, 'input>> {
        Self::set_next(&mut self, None);
        let bytes = self.flow.steps.get_mut(self.index).unwrap();

        unsafe {
            *bytes.get_unchecked_mut(0x0) = 2;
            *bytes.get_unchecked_mut(0xC) = count;
            *bytes.get_unchecked_mut(0xE) = branch_index;
        }

        Some(BranchMut(self))
    }

    // FIXME this is actually setting the command, not the kind
    fn set_kind(&mut self, kind: u16) {
        unsafe {
            self.flow
                .steps
                .get_mut(self.index)
                .unwrap()
                .get_unchecked_mut(0xA..0xC)
                .copy_from_slice(&kind.to_le_bytes());
        }
    }

    fn set_arg1(&mut self, value: u8) {
        unsafe {
            *self.flow.steps.get_mut(self.index).unwrap().get_unchecked_mut(0x1) = value;
        }
    }

    fn set_value(&mut self, value: u32) {
        unsafe {
            self.flow.steps.get_mut(self.index).unwrap().get_unchecked_mut(4..8).copy_from_slice(&value.to_le_bytes());
        }
    }

    fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        let next = next.into().unwrap_or(0xFFFF);
        unsafe {
            self.flow.steps.get_mut(self.index).unwrap().get_unchecked_mut(8..0xA).copy_from_slice(&next.to_le_bytes());
        }
    }

    fn set_command(&mut self, command: u16) {
        unsafe {
            self.flow
                .steps
                .get_mut(self.index)
                .unwrap()
                .get_unchecked_mut(0xA..0xC)
                .copy_from_slice(&command.to_le_bytes());
        }
    }

    fn set_count(&mut self, count: u16) {
        unsafe {
            self.flow
                .steps
                .get_mut(self.index)
                .unwrap()
                .get_unchecked_mut(0xC..0xE)
                .copy_from_slice(&count.to_le_bytes());
        }
    }
}

#[derive(Debug)]
pub struct BranchMut<'flow, 'input>(StepMut<'flow, 'input>);

impl<'flow, 'input> BranchMut<'flow, 'input> {
    pub fn set_kind(&mut self, kind: u16) {
        self.0.set_kind(kind);
    }

    pub fn set_arg1(&mut self, value: u8) {
        self.0.set_arg1(value);
    }

    pub fn set_value(&mut self, value: u32) {
        self.0.set_value(value);
    }

    pub fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        self.0.set_next(next);
    }

    pub fn set_command(&mut self, command: u16) {
        self.0.set_command(command);
    }

    pub fn set_count(&mut self, count: u16) {
        self.0.set_count(count);
    }

    pub fn set_branch<N>(&mut self, index: u16, to: N) -> Result<()>
    // index:4, to:6
    where
        N: Into<Next>,
    {
        typedef! { struct Inner: FromBytes<'_> [STEP_LEN] {
            [0xC] count: u16,
            [0xE] branch: u16,
        }}
        let inner: Inner = Inner::from_bytes(self.0.flow.steps.get_mut(self.0.index).unwrap());
        if index < inner.count {
            let index = inner.branch + index;
            let branch = self.0.flow.branches.get_mut(index).ok_or_else(|| Error::new("Invalid branch index."))?;
            branch.copy_from_slice(&to.into().unwrap_or(0xFFFF).to_le_bytes());
            Ok(())
        } else {
            Err(Error::new("Invalid branch index."))
        }
    }
}

#[derive(Debug)]
pub struct ActionMut<'flow, 'input>(StepMut<'flow, 'input>);

impl<'flow, 'input> ActionMut<'flow, 'input> {
    pub fn set_kind(&mut self, kind: u16) {
        self.0.set_kind(kind);
    }

    pub fn set_arg1(&mut self, value: u8) {
        self.0.set_arg1(value);
    }

    pub fn set_value(&mut self, value: u32) {
        self.0.set_value(value);
    }

    pub fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        self.0.set_next(next);
    }

    pub fn set_command(&mut self, command: u16) {
        self.0.set_command(command);
    }

    pub fn set_count(&mut self, count: u16) {
        self.0.set_count(count);
    }
}

#[derive(Debug)]
pub struct TextMut<'flow, 'input>(StepMut<'flow, 'input>);

impl<'flow, 'input> TextMut<'flow, 'input> {
    pub fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        self.0.set_next(next);
    }
}

#[derive(Debug)]
pub struct StartMut<'flow, 'input>(StepMut<'flow, 'input>);

impl<'flow, 'input> StartMut<'flow, 'input> {
    pub fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        self.0.set_next(next);
    }
}

#[derive(Debug)]
pub struct GotoMut<'flow, 'input>(StepMut<'flow, 'input>);

impl<'flow, 'input> GotoMut<'flow, 'input> {
    pub fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        self.0.set_next(next);
    }
}

type BranchesMut<'input> = List<RefMut<'input>, BRANCH_LEN>;

#[derive(Debug)]
struct List<T, const SIZE: usize> {
    count: u16,
    inner: T,
}

impl<'input, const SIZE: usize> List<Ref<'input>, SIZE> {
    fn new(count: u16, inner: Ref<'input>) -> Option<Self> {
        let len = SIZE * count as usize;
        if inner.len() < len {
            None
        } else {
            Some(Self { count, inner: Ref::map(inner, |inner| unsafe { inner.get_unchecked(0..len) }) })
        }
    }

    fn get(&self, index: u16) -> Option<RefSized<'input, SIZE>> {
        (index <= self.count).then(|| {
            Ref::map(Ref::clone(&self.inner), |inner| {
                let start = SIZE * index as usize;
                unsafe {
                    let ptr = inner.get_unchecked(start..start + SIZE).as_ptr() as *const [u8; SIZE];
                    &*ptr
                }
            })
        })
    }

    fn iter<'s>(&'s self) -> impl Iterator<Item = RefSized<'input, SIZE>> + 's {
        (0..self.count).map(move |index| {
            let index = index as usize;
            Ref::map(Ref::clone(&self.inner), |inner| unsafe {
                let start = SIZE * index;
                let ptr = inner.get_unchecked(start..start + SIZE).as_ptr() as *const [u8; SIZE];
                &*ptr
            })
        })
    }
}

impl<'input, const SIZE: usize> List<RefMut<'input>, SIZE> {
    fn new(count: u16, inner: RefMut<'input>) -> Option<Self> {
        let len = SIZE * count as usize;
        if inner.len() < len {
            None
        } else {
            Some(Self { count, inner: unsafe { inner.get_unchecked_mut(0..len) } })
        }
    }

    fn get_mut(&mut self, index: u16) -> Option<&mut [u8; SIZE]> {
        if index <= self.count {
            let start = SIZE * index as usize;
            unsafe {
                let ptr = self.inner.get_unchecked_mut(start..start + SIZE).as_mut_ptr() as *mut [u8; SIZE];
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }
}

fn get_flw(flw: &[u8]) -> Result<(u16, u16, usize)> {
    typedef! { struct Header: FromBytes<'_> [0x10] {
        [0] steps: u16,
        [2] branches: u16,
    }}
    let (Header { steps, branches }, _) = Header::try_from_slice(flw)?;
    let end = 0x10 + (0x10 * steps) + (2 * branches);
    if end as usize > flw.len() {
        Err(Error::new("Not enough data."))
    } else {
        Ok((steps, branches, 0x10 * steps as usize))
    }
}

fn next(index: u16) -> Next {
    (index != 0xFFFF).then_some(index)
}

const MSGFLWBN: &[u8; 8] = b"MsgFlwBn";
const FLW3: &[u8; 4] = b"FLW3";
const HEADER_LEN: usize = 0x4;
const STEP_LEN: usize = 0x10;
const BRANCH_LEN: usize = 2;
