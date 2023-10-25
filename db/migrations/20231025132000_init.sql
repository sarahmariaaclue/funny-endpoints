CREATE TABLE public.beer_brand ( --IF NOT EXISTS
            id BIGSERIAL PRIMARY KEY,
            "name" VARCHAR(255) NOT NULL,
            city VARCHAR(255) NOT NULL
        )