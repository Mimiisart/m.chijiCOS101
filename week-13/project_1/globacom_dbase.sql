--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    customer_id integer NOT NULL,
    customer_name text NOT NULL,
    custumer_age integer NOT NULL,
    customer_email text NOT NULL,
    customer_number character(15) NOT NULL,
    employee_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    department_managerid integer NOT NULL,
    department_number integer,
    department_name text NOT NULL,
    department_location text,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    project_no integer NOT NULL,
    project_name text NOT NULL,
    project_duration text NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    department_number integer,
    staff_salary real,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (customer_id, customer_name, custumer_age, customer_email, customer_number, employee_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112    	102	5
111	Lilian Jaiya	43	i_jaiye@gmail.com	8055185341     	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813    	107	10
113	Phillip Akonjo	41	p_akonjo@gmail.com	09052356772    	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551    	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774    	117	11
116	Adams Bre	33	a_bree@gmail.com	08056765424    	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	08055638891    	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	07056774423    	117	11
119	Lawal Tamire	35	i_tamire@gmail.com	09052111101    	107	5
120	James Job	44	j_job@gmail.com	08059693919    	100	8
121	Mathew jakande	21	m_jakande@gmail.com	07051232144    	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923    	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350mb	2	200
2	1.8gb	14	500
3	3.9gb	30	1000
4	7.5gb	30	1500
5	9.2gb	30	2000
6	10.8gb	30	2500
7	14gb	30	3000
8	18gb	30	4000
9	24gb	30	5000
10	29.9gb	30	8000
11	50gb	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (department_managerid, department_number, department_name, department_location, pno) FROM stdin;
108	1	Administration	ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.i	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (project_no, project_name, project_duration, project_managerid) FROM stdin;
11	A	nine months	102
22	B	fourteen months	97
33	C	sixteen months	120
55	E	nine months	107
44	D	twenty_five months	108
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, department_number, staff_salary, age, mobile) FROM stdin;
100	mustapha\n ali	3	175000	32	09123456745
107	alokwe martins	7	380000	48	07090082812
97	dankade aminat	5	550000	40	09023688832
108	josiah joshua	1	120000	30	08053189131
102	mankinde mary	2	450000	55	09023487830
120	adeleke jane	4	200000	38	07061045862
122	osahon mark	6	320000	44	08022289842
104	kuti lawal	1	750000	35	09133614345
117	sulieman ajayi	3	800000	50	07030089981
\.


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (department_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

