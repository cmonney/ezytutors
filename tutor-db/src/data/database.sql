create database ezytutors;

create user truuser with password 'mypassword';

GRANT ALL PRIVILEGES ON DATABASE ezytutors TO truuser;

GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO truuser;

GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO truuser;

drop table if exists ezy_course_c4;

create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

insert into ezy_course_c4
(tutor_id, course_name, posted_time)
values (1,'First Course', '2023-12-14 23:56:00');

insert into ezy_course_c4
(tutor_id, course_name, posted_time)
values (1,'Second Course', '2023-12-13 05:30:00');