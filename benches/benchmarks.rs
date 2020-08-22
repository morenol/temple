use criterion::{criterion_group, criterion_main, Criterion};

use temple::value::{Value, ValuesList, ValuesMap};
use temple::{RealFileSystem, TemplateEnv};

pub fn raw_text_benchmark(c: &mut Criterion) {
    c.bench_function("raw_text", |b| {
        b.iter(|| {
            let mut temp_env = TemplateEnv::default();
            let handler = RealFileSystem::new("benches/templates/".to_string());
            temp_env.add_filesystem_handler(Box::new(handler)).unwrap();
            let template = temp_env.load_template("raw_test").unwrap();
            let context = ValuesMap::default();
            template.render_as_string(context).unwrap();
        })
    });
}

pub fn raw_text_with_var_benchmark(c: &mut Criterion) {
    c.bench_function("raw_text_with_var", |b| {
        b.iter(|| {
            let mut temp_env = TemplateEnv::default();
            let handler = RealFileSystem::new("benches/templates/".to_string());
            temp_env.add_filesystem_handler(Box::new(handler)).unwrap();
            let template = temp_env.load_template("raw_block").unwrap();
            let mut context = ValuesMap::default();
            context.insert(
                "message".to_string(),
                Value::String("Hello World!".to_string()),
            );

            template.render_as_string(context).unwrap();
        })
    });
}

pub fn for_benchmark(c: &mut Criterion) {
    c.bench_function("for_benchmark", |b| {
        b.iter(|| {
            let mut temp_env = TemplateEnv::default();
            let handler = RealFileSystem::new("benches/templates/".to_string());
            temp_env.add_filesystem_handler(Box::new(handler)).unwrap();
            let template = temp_env.load_template("for").unwrap();
            let mut context = ValuesMap::default();
            let mut value_list = ValuesList::default();
            for i in 0..20 {
                value_list.push(Value::Integer(i));
            }
            context.insert("list".to_string(), Value::ValuesList(value_list));

            template.render_as_string(context).unwrap();
        })
    });
}

criterion_group!(
    benches,
    raw_text_benchmark,
    raw_text_with_var_benchmark,
    for_benchmark
);
criterion_main!(benches);
