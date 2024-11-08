--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
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
-- Name: allergies; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.allergies (
    allergy_id integer NOT NULL,
    name text NOT NULL,
    symptoms text NOT NULL
);


ALTER TABLE public.allergies OWNER TO postgres;

--
-- Name: patients; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.patients (
    patients_id integer NOT NULL,
    last_name text NOT NULL,
    first_name text NOT NULL,
    e_mail character varying(50) NOT NULL,
    date_of_birth character varying(15) NOT NULL,
    physician_id integer NOT NULL,
    allergy_id integer NOT NULL
);


ALTER TABLE public.patients OWNER TO postgres;

--
-- Name: physicians; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.physicians (
    physician_id integer NOT NULL,
    last_name text NOT NULL,
    first_name text NOT NULL,
    e_mail character varying(50) NOT NULL,
    specialization text NOT NULL
);


ALTER TABLE public.physicians OWNER TO postgres;

--
-- Data for Name: allergies; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.allergies (allergy_id, name, symptoms) FROM stdin;
1	Low sugar	Feeling shaky or trembling
2	Low cholesterol	Change in your mood, sleep, or eating patterns
3	Diabetes	Increased thirst
4	Anaphylaxis	Rapid fall in blood pressure
5	Fish	Vomiting and diarrhea
\.


--
-- Data for Name: patients; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.patients (patients_id, last_name, first_name, e_mail, date_of_birth, physician_id, allergy_id) FROM stdin;
1	Agada	Simon	a-simon@gmail.com	12/3/1992	2	3
2	Fagbemi	Tina	f-tina@gmail.com	3/12/1989	3	1
3	Dalegi	Valerie	d-valerie@gmail.com	11/1/1993	1	2
4	Salami	Samuel	s-samuel@gmail.com	21/7/1998	2	5
5	Oghenero	Feji	o-feji@gmail.com	10/6/1991	3	2
6	Mustapha	Kabir	m-kabir@gmail.com	13/05/1990	2	4
7	Alokwe	Jane	a-jane@gmail.com	28/11/1988	3	1
8	Makman	Ali	m-ali@gmail.com	1/12/2000	1	3
9	Okonkwo	Chisom	o-chisom@gmail.com	15/11/1999	1	5
10	Eze	Agatha	e-agatha@gmail.com	22/04/1996	2	1
\.


--
-- Data for Name: physicians; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.physicians (physician_id, last_name, first_name, e_mail, specialization) FROM stdin;
1	Audu	Gloria	g-audu@patron.org	Orthopedic
2	Seidu	Ahmed	a-seidu@patron.org	Surgery
3	Gbenga	Mildred	m-gbenga@patron.org	Pediatrics
\.


--
-- Name: allergies allergies_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.allergies
    ADD CONSTRAINT allergies_pkey PRIMARY KEY (allergy_id);


--
-- Name: patients patients_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.patients
    ADD CONSTRAINT patients_pkey PRIMARY KEY (patients_id);


--
-- Name: physicians physicians_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.physicians
    ADD CONSTRAINT physicians_pkey PRIMARY KEY (physician_id);


--
-- PostgreSQL database dump complete
--

