CREATE TABLE IF NOT EXISTS public.jobs
(
    id BIGSERIAL NOT NULL,
    "timestamp" timestamp without time zone NOT NULL DEFAULT now(),
    commands bigint NOT NULL,
    result bigint NOT NULL,
    duration double precision NOT NULL,
    CONSTRAINT jobs_pkey PRIMARY KEY (id)
)