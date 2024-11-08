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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    d_no integer NOT NULL,
    dept_managerid integer NOT NULL,
    d_name text NOT NULL,
    d_location text NOT NULL,
    p_no integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (d_no, dept_managerid, d_name, d_location, p_no) FROM stdin;
1	108	Administration	Ikeja	44
2	101	Account	Egbeda	11
3	100	Packaging	Ajah	44
4	120	Research	V.I	33
5	97	Account	Magodo	22
6	122	Operations	Mile 2	44
7	107	Packaging	Ketu	55
\.


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (d_no);


--
-- PostgreSQL database dump complete
--

