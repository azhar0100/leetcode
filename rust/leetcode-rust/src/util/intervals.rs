// use std::cmp::Ordering;

// pub trait IntervalAble: Ord + PartialOrd + PartialEq + Eq + Clone {}
// impl<T> IntervalAble for T where T: Ord + PartialOrd + PartialEq + Eq + Clone {}

// #[derive(Debug, Clone)]
// pub enum IntervalBoundaryQuality {
//     Closed,
//     Open,
// }

// #[derive(Debug, Clone)]
// pub enum IntervalBoundary<T> {
//     Closed(T),
//     Open(T),
// }

// impl<T> IntervalBoundary<T>
// where
//     T: IntervalAble,
// {
//     fn quality(&self) -> IntervalBoundaryQuality {
//         match self {
//             IntervalBoundary::Closed(_) => IntervalBoundaryQuality::Closed,
//             IntervalBoundary::Open(_) => IntervalBoundaryQuality::Open,
//         }
//     }
// }

// impl<T: IntervalAble> PartialEq for IntervalBoundary<T> {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Closed(l0), Self::Closed(r0)) => l0 == r0,
//             (Self::Open(l0), Self::Open(r0)) => l0 == r0,
//             _ => false,
//         }
//     }
// }

// impl<T: IntervalAble> PartialOrd for IntervalBoundary<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let self_val = match self {
//             IntervalBoundary::Closed(val) => val,
//             IntervalBoundary::Open(val) => val,
//         };
//         let other_val = match other {
//             IntervalBoundary::Closed(val) => val,
//             IntervalBoundary::Open(val) => val,
//         };
//         self_val.partial_cmp(other_val).map(|cmp| match cmp {
//             Ordering::Equal => match (self, other) {
//                 (IntervalBoundary::Closed(_), IntervalBoundary::Open(_)) => Ordering::Greater,
//                 (IntervalBoundary::Open(_), IntervalBoundary::Closed(_)) => Ordering::Less,
//                 _ => Ordering::Equal,
//             },
//             Ordering::Less => cmp,
//             Ordering::Greater => cmp,
//         })
//     }
// }

// impl<T> Eq for IntervalBoundary<T> where T: IntervalAble {}
// impl<T> Ord for IntervalBoundary<T>
// where
//     T: IntervalAble,
// {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }
// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
// pub struct IntervalStart<T>(IntervalBoundary<T>)
// where
//     T: IntervalAble;
// #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
// pub struct IntervalEnd<T>(IntervalBoundary<T>)
// where
//     T: IntervalAble;

// #[derive(Debug, Clone)]
// pub enum IntervalElement<T>
// where
//     T: IntervalAble,
// {
//     Start(IntervalStart<T>),
//     End(IntervalEnd<T>),
// }

// impl<T> PartialEq for IntervalElement<T>
// where
//     T: IntervalAble,
// {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (IntervalElement::Start(a), IntervalElement::Start(b)) => match (&a.0, &b.0) {
//                 (IntervalBoundary::Closed(a), IntervalBoundary::Closed(b)) => a == b,
//                 (IntervalBoundary::Closed(_), IntervalBoundary::Open(_)) => todo!(),
//                 (IntervalBoundary::Open(_), IntervalBoundary::Closed(_)) => todo!(),
//                 (IntervalBoundary::Open(_), IntervalBoundary::Open(_)) => todo!(),
//             },
//             (IntervalElement::End(a), IntervalElement::End(b)) => a.0.eq(&b.0),
//             _ => false,
//         }
//     }
// }

// impl<T> Eq for IntervalElement<T> where T: IntervalAble {}

// impl<T> PartialOrd for IntervalElement<T>
// where
//     T: IntervalAble,
// {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match (self, other) {
//             (IntervalElement::End(a), IntervalElement::Start(b)) => Some(a.0.cmp(&b.0)),
//             (IntervalElement::Start(a), IntervalElement::Start(b)) => Some(a.0.cmp(&b.0)),
//             (IntervalElement::End(a), IntervalElement::End(b)) => Some(a.0.cmp(&b.0)),
//             (IntervalElement::Start(a), IntervalElement::End(b)) => Some(a.0.cmp(&b.0)),
//         }
//     }
// }

// impl<T> Ord for IntervalElement<T>
// where
//     T: IntervalAble,
//     T: IntervalAble,
// {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

// pub struct Interval<T>
// where
//     T: IntervalAble,
// {
//     start: IntervalStart<T>,
//     end: IntervalEnd<T>,
// }

// impl<T> Interval<T>
// where
//     T: IntervalAble,
// {
//     pub fn new(start: T, end: T, closed: (bool, bool)) -> Self {
//         let (start_closed, end_closed) = closed;
//         let start_element = match start_closed {
//             true => IntervalStart(IntervalBoundary::Closed(start)),
//             false => IntervalStart(IntervalBoundary::Open(start)),
//         };
//         let end_element = match end_closed {
//             true => IntervalEnd(IntervalBoundary::Closed(end)),
//             false => IntervalEnd(IntervalBoundary::Open(end)),
//         };
//         Self {
//             start: start_element,
//             end: end_element,
//         }
//     }

//     pub fn from_interval_start_and_end(
//         start: IntervalElement<T>,
//         end: IntervalElement<T>,
//     ) -> Option<Self> {
//         match (&start, &end) {
//             (IntervalElement::Start(interval_start), IntervalElement::End(interval_end)) => {
//                 let (qual_a, val_a) = (
//                     interval_start.0.quality(),
//                     match &interval_start.0 {
//                         IntervalBoundary::Closed(val) => val,
//                         IntervalBoundary::Open(val) => val,
//                     },
//                 );
//                 let (qual_b, val_b) = (
//                     interval_end.0.quality(),
//                     match &interval_end.0 {
//                         IntervalBoundary::Closed(val) => val,
//                         IntervalBoundary::Open(val) => val,
//                     },
//                 );
//                 let cmp = val_a.cmp(&val_b);
//                 match (qual_a, qual_b, cmp) {
//                     (
//                         IntervalBoundaryQuality::Closed,
//                         IntervalBoundaryQuality::Closed,
//                         Ordering::Equal,
//                     ) => Some(Self {
//                         start: interval_start.clone(),
//                         end: interval_end.clone(),
//                     }),
//                     (_, _, Ordering::Equal) => None,
//                     (_, _, Ordering::Less) => Some(Self {
//                         start: interval_start.clone(),
//                         end: interval_end.clone(),
//                     }),
//                     (_, _, Ordering::Greater) => None,
//                 }
//             }
//             _ => None,
//         }
//     }

//     pub fn intersection(&self, other: &Self) -> Option<Interval<T>> {
//         let start_max = IntervalElement::Start(self.start.clone())
//             .max(IntervalElement::Start(other.start.clone()));
//         let end_min =
//             IntervalElement::End(self.end.clone()).min(IntervalElement::End(other.end.clone()));
//         match (&start_max, &end_min) {
//             (IntervalElement::Start(interval_start), IntervalElement::End(interval_end)) => {
//                 let (qual_a, val_a) = (
//                     interval_start.0.quality(),
//                     match &interval_start.0 {
//                         IntervalBoundary::Closed(val) => val,
//                         IntervalBoundary::Open(val) => val,
//                     },
//                 );
//                 let (qual_b, val_b) = (
//                     interval_end.0.quality(),
//                     match &interval_end.0 {
//                         IntervalBoundary::Closed(val) => val,
//                         IntervalBoundary::Open(val) => val,
//                     },
//                 );
//                 let cmp = val_a.cmp(&val_b);
//                 match (qual_a, qual_b, cmp) {
//                     (
//                         IntervalBoundaryQuality::Closed,
//                         IntervalBoundaryQuality::Closed,
//                         Ordering::Equal,
//                     ) => Some(Self {
//                         start: interval_start.clone(),
//                         end: interval_end.clone(),
//                     }),
//                     (_, _, Ordering::Equal) => None,
//                     (_, _, Ordering::Less) => Some(Self {
//                         start: interval_start.clone(),
//                         end: interval_end.clone(),
//                     }),
//                     (_, _, Ordering::Greater) => None,
//                 }
//             }
//             _ => None,
//         }
//     }
// }

// pub fn find_interval_intersections<T>(intervals: &[Interval<T>]) -> Vec<Vec<usize>>
// where
//     T: IntervalAble,
// {
//     // let interval_elements =
//     todo!()
// }
