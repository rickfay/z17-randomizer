use bytey::*;

use crate::{
    files::{msgbn::MsgBn, FromFile},
    Error, Result,
};

type Ref<'input> = ::core::cell::Ref<'input, [u8]>;
type RefSized<'input, const N: usize> = ::core::cell::Ref<'input, [u8; N]>;
type RefMut<'input> = &'input mut [u8];

type Next = Option<u16>;

#[derive(Debug)]
pub struct Flow<'input> {
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
        let msgbn = MsgBn::<Ref<'input>, 2>::try_read(input, MSGFLWBN)?;
        let flw = msgbn.get(FLW3).ok_or_else(|| Error::new("No FLW3"))?;
        let (step_ct, branch_ct, index) = get_flw(&*flw)?;
        let (steps, branches) = Ref::map_split(flw, |flw| flw[0x10..].split_at(index));
        let steps =
            List::<Ref, STEP_LEN>::new(step_ct, steps).ok_or_else(|| Error::new("unimpl33"))?;
        let branches = Branches(
            List::<Ref, BRANCH_LEN>::new(branch_ct, branches)
                .ok_or_else(|| Error::new("unimpl33"))?,
        );
        Ok(Self { steps, branches })
    }
}

#[derive(Debug)]
pub struct Steps<'flow, 'input>(&'flow Flow<'input>);

impl<'flow, 'input> Steps<'flow, 'input> {
    pub fn get(&self, index: u16) -> Option<Result<Step>> {
        self.0
            .steps
            .get(index)
            .map(|bytes| Step::from_bytes(&bytes, &self.0.branches))
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<Step>> + '_ {
        self.0
            .steps
            .iter()
            .map(move |bytes| Step::from_bytes(&bytes, &self.0.branches))
    }
}

#[derive(Debug)]
pub enum Step {
    Text {
        next: Next,
    },
    Branch {
        command: Branch,
        branches: Vec<Next>,
    },
    Action {
        command: Action,
        next: Next,
    },
    Start {
        next: Next,
    },
    Goto {
        next: Next,
    },
}

impl Step {
    fn from_bytes(bytes: &[u8; STEP_LEN], branches: &Branches) -> Result<Self> {
        bytey::typedef! { struct Inner: FromBytes<'_> [STEP_LEN] {
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
            2 => Self::branch(
                branches,
                Branch::new(step.command, step.value),
                step.count,
                step.branch,
            ),
            3 => Ok(Self::Action {
                command: Action::new(step.command, step.value),
                next,
            }),
            4 => Ok(Self::Start { next }),
            5 => Ok(Self::Goto { next }),
            kind => Err(Error::new(format!("Invalid flow value: {:X}.", kind))),
        }
    }

    fn branch(branches: &Branches, command: Branch, count: u16, branch: u16) -> Result<Self> {
        let branches = branches
            .iter()
            .skip(branch as usize)
            .take(count as usize)
            .map(next)
            .collect::<Vec<_>>();
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
        let flw = msgbn
            .into_section(FLW3)
            .ok_or_else(|| Error::new("No FLW3"))?;
        let (step_ct, branch_ct, index) = get_flw(&flw)?;
        let (steps, branches) = flw[0x10..].split_at_mut(index);
        let steps =
            List::<RefMut, STEP_LEN>::new(step_ct, steps).ok_or_else(|| Error::new("unimpl33"))?;
        let branches = List::<RefMut, BRANCH_LEN>::new(branch_ct, branches)
            .ok_or_else(|| Error::new("unimpl33"))?;
        Ok(Self { steps, branches })
    }
}

#[derive(Debug)]
pub struct StepMut<'flow, 'input> {
    flow: &'flow mut FlowMut<'input>,
    index: u16,
}

impl<'flow, 'input> StepMut<'flow, 'input> {
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
            *self
                .flow
                .steps
                .get_mut(self.index)
                .unwrap()
                .get_unchecked_mut(0) = 3;
            Some(ActionMut(self))
        }
    }

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

    fn set_value(&mut self, value: u32) {
        unsafe {
            self.flow
                .steps
                .get_mut(self.index)
                .unwrap()
                .get_unchecked_mut(4..8)
                .copy_from_slice(&value.to_le_bytes());
        }
    }

    fn set_next<N>(&mut self, next: N)
    where
        N: Into<Next>,
    {
        let next = next.into().unwrap_or(0xFFFF);
        unsafe {
            self.flow
                .steps
                .get_mut(self.index)
                .unwrap()
                .get_unchecked_mut(8..0xA)
                .copy_from_slice(&next.to_le_bytes());
        }
    }
}

#[derive(Debug)]
pub struct BranchMut<'flow, 'input>(StepMut<'flow, 'input>);

impl<'flow, 'input> BranchMut<'flow, 'input> {
    pub fn set_kind(&mut self, kind: u16) {
        self.0.set_kind(kind);
    }

    pub fn set_value(&mut self, value: u32) {
        self.0.set_value(value);
    }

    pub fn set_branch<N>(&mut self, index: u16, to: N) -> Result<()>
    where
        N: Into<Next>,
    {
        bytey::typedef! { struct Inner: FromBytes<'_> [STEP_LEN] {
            [0xC] count: u16,
            [0xE] branch: u16,
        }}
        let inner = Inner::from_bytes(self.0.flow.steps.get_mut(self.0.index).unwrap());
        if index < inner.count {
            let index = inner.branch + index;
            let branch = self
                .0
                .flow
                .branches
                .get_mut(index)
                .ok_or_else(|| Error::new("Invalid branch index."))?;
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

    pub fn set_value(&mut self, value: u32) {
        self.0.set_value(value);
    }

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
            Some(Self {
                count,
                inner: Ref::map(inner, |inner| unsafe { inner.get_unchecked(0..len) }),
            })
        }
    }

    fn get(&self, index: u16) -> Option<RefSized<'input, SIZE>> {
        (index <= self.count).then(|| {
            Ref::map(Ref::clone(&self.inner), |inner| {
                let start = SIZE * index as usize;
                unsafe {
                    let ptr =
                        inner.get_unchecked(start..start + SIZE).as_ptr() as *const [u8; SIZE];
                    &*ptr
                }
            })
        })
    }

    fn iter<'s>(&'s self) -> impl Iterator<Item = RefSized<'input, SIZE>> + 's {
        (0..self.count).map(move |index| {
            let index = index as usize;
            Ref::map(Ref::clone(&self.inner), |inner| unsafe {
                let start = SIZE * index as usize;
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
            Some(Self {
                count,
                inner: unsafe { inner.get_unchecked_mut(0..len) },
            })
        }
    }

    fn get_mut(&mut self, index: u16) -> Option<&mut [u8; SIZE]> {
        if index <= self.count {
            let start = SIZE * index as usize;
            unsafe {
                let ptr = self
                    .inner
                    .get_unchecked_mut(start..start + SIZE)
                    .as_mut_ptr() as *mut [u8; SIZE];
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }
}

fn get_flw(flw: &[u8]) -> Result<(u16, u16, usize)> {
    bytey::typedef! { struct Header: FromBytes<'_> [0x10] {
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
    (index != 0xFFFF).then(|| index)
}

const MSGFLWBN: &[u8; 8] = b"MsgFlwBn";
const FLW3: &[u8; 4] = b"FLW3";
const STEP_LEN: usize = 0x10;
const BRANCH_LEN: usize = 2;
