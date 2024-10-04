#![allow(dead_code)]

use std::collections::HashMap;

use vate::{
    path, Accessor, CollectionIterate, Collector, Everything, Exit, IteratorIndexed,
    IteratorKeyedPair, Nested, Report, Validate, Validator,
};

/// Mapping of exam question to answer. The answers can represent
/// both the student answers or the correct answers.
type ExamAnswers = HashMap<u8, char>;

/// Re[resents an exam session.
#[derive(Validate)]
#[vate(data = ExamAnswers, error = ExamError)]
struct ExamSession {
    /// The proctor of the exam. This field is not validated.
    proctor: String,
    /// Validate the nested objects.
    #[vate(CollectionIterate(IteratorIndexed(Nested)))]
    submissions: Vec<ExamSubmission>,
}

/// Represents an exam submission by a student.
#[derive(Validate)]
#[vate(data = ExamAnswers, error = ExamError)]
struct ExamSubmission {
    student_name: String,
    preferred_name: Option<String>,
    #[vate(CollectionIterate(IteratorKeyedPair(CorrectAnswer)))]
    answers: ExamAnswers,
}

/// Errors relating to the exam.
#[derive(Debug)]
enum ExamError {
    /// Something went wrong with the backend, and a question that was answered
    /// isn't in the answer key.
    InvalidQuestion(u8),
}

struct CorrectAnswer;

impl Validator<(&u8, &char), ExamAnswers, ExamError> for CorrectAnswer {
    fn run<C: Collector<ExamError>>(
        &self,
        accessor: Accessor,
        target: &(&u8, &char),
        data: &ExamAnswers,
        parent_report: &mut Report<ExamError>,
    ) -> Result<(), Exit<ExamError>> {
        let (question_number, student_answer) = *target;

        let mut child_report = Report::new(accessor);

        match student_answer {
            'A' | 'B' | 'C' | 'D' => {
                if let Some(correct_answer) = data.get(question_number) {
                    if student_answer == correct_answer {
                        child_report.validity = Ok(true);
                    } else {
                        child_report.validity = Ok(false);
                        child_report.message = format!(
                            "has the incorrect answer {student_answer} instead of {correct_answer}"
                        );
                    }
                } else {
                    child_report.validity = Err(ExamError::InvalidQuestion(*question_number));
                }
            }
            _ => {
                // If the student answer is not valid, mark as incorrect.
                child_report.validity = Ok(false);
                child_report.message = format!("has the answer {student_answer}, which is invalid");
            }
        }
        parent_report.push_child::<C>(child_report)
    }
}

fn main() {
    let correct_answers = ExamAnswers::from([(1, 'A'), (2, 'B'), (3, 'B'), (4, 'A')]);

    let session = ExamSession {
        proctor: String::from("Professor Ross. E. Frop"),
        submissions: vec![
            ExamSubmission {
                student_name: String::from("Ivy Lee Garad"),
                preferred_name: None,
                answers: ExamAnswers::from([(1, 'A'), (2, 'B'), (3, 'B'), (4, 'A')]),
            },
            ExamSubmission {
                student_name: String::from("Stewart Dent"),
                preferred_name: Some(String::from("Stew")),
                answers: ExamAnswers::from([
                    (1, 'A'),
                    (2, 'B'),
                    (3, 'C'), // Incorrect answer.
                    (4, 'A'),
                ]),
            },
            ExamSubmission {
                student_name: String::from("Tyler Poe"),
                preferred_name: Some(String::from("Ty")),
                answers: ExamAnswers::from([
                    (1, 'A'),
                    (2, 'B'),
                    (3, 'X'), // Incorrect answer (typo).
                    (4, 'A'),
                ]),
            },
            ExamSubmission {
                student_name: String::from("Max Marks"),
                preferred_name: None,
                answers: ExamAnswers::from([(1, 'A'), (2, 'B'), (3, 'B'), (4, 'A')]),
            },
            ExamSubmission {
                student_name: String::from("Max Marks"),
                preferred_name: None,
                answers: ExamAnswers::from([(1, 'A'), (2, 'B'), (3, 'B'), (4, 'A')]),
            },
            ExamSubmission {
                student_name: String::from("Phil Yer"),
                preferred_name: None,
                answers: ExamAnswers::from([
                    (1, 'C'), // Incorrect answer.
                    (2, 'C'), // Incorrect answer.
                    (3, 'C'), // Incorrect answer.
                    (4, 'C'), // Incorrect answer.
                ]),
            },
            ExamSubmission {
                student_name: String::from("Max Marks"),
                preferred_name: None,
                answers: ExamAnswers::from([(1, 'A'), (2, 'B'), (3, 'B'), (4, 'A')]),
            },
            ExamSubmission {
                student_name: String::from("Paige Turner"),
                preferred_name: None,
                answers: ExamAnswers::from([
                    (1, 'A'),
                    (2, 'B'),
                    (3, 'B'),
                    (5, 'A'), // Error, something went wrong and there is an invalid question number.
                ]),
            },
            ExamSubmission {
                student_name: String::from("Dean List"),
                preferred_name: None,
                answers: ExamAnswers::from([(1, 'A'), (2, 'B'), (3, 'B'), (4, 'A')]),
            },
        ],
    };

    let mut report = Report::new(Accessor::Root("session"));
    let _ = session.validate::<Everything>(&correct_answers, &mut report);

    assert!(report
        .is_invalid_at_path(path!(session.submissions[1].answers["3"]))
        .unwrap());
    assert!(report
        .is_invalid_at_path(path!(session.submissions[2].answers["3"]))
        .unwrap());
    assert!(report
        .is_invalid_at_path(path!(session.submissions[5].answers["1"]))
        .unwrap());
    assert!(report
        .is_invalid_at_path(path!(session.submissions[5].answers["2"]))
        .unwrap());
    assert!(report
        .is_invalid_at_path(path!(session.submissions[5].answers["3"]))
        .unwrap());
    assert!(report
        .is_invalid_at_path(path!(session.submissions[5].answers["4"]))
        .unwrap());
    assert!(report
        .is_error_at_path(path!(session.submissions[7].answers["5"]))
        .unwrap());

    println!("{report:#?}");
}
