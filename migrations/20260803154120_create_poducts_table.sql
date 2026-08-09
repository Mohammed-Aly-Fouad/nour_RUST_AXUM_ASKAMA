-- ========================================================
-- 3. Table: public.products
-- ========================================================

-- DROP TABLE IF EXISTS public.products CASCADE;

CREATE TABLE IF NOT EXISTS public.products
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    category_id bigint NOT NULL,
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    name_ar character varying(255) COLLATE pg_catalog."default" NOT NULL,
    notes text COLLATE pg_catalog."default",
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_at timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT products_pkey PRIMARY KEY (id),
    CONSTRAINT fk_products_category FOREIGN KEY (category_id)
        REFERENCES public.categories (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.products OWNER to mohammed;

-- Indexes for products
CREATE INDEX IF NOT EXISTS idx_products_category_id
    ON public.products USING btree
    (category_id ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE UNIQUE INDEX IF NOT EXISTS idx_products_unique_name_lower
    ON public.products USING btree
    (lower(name::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE UNIQUE INDEX IF NOT EXISTS idx_products_unique_name_ar_lower
    ON public.products USING btree
    (lower(name_ar::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

-- Trigger for products
CREATE OR REPLACE TRIGGER update_products_modtime
    BEFORE UPDATE ON public.products
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();