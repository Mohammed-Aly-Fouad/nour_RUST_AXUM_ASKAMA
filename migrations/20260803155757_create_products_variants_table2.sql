-- ========================================================
-- 0. Helper Functions
-- ========================================================

CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 1. Table: public.product_variants
-- ========================================================

CREATE TABLE IF NOT EXISTS public.product_variants
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    product_id bigint NOT NULL,
    brand_id bigint NOT NULL,
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    name_ar character varying(255) COLLATE pg_catalog."default" NOT NULL,
    sku character varying(100) COLLATE pg_catalog."default" NOT NULL,
    barcode character varying(100) COLLATE pg_catalog."default",
    shelf_location character varying(100) COLLATE pg_catalog."default",
    stock_quantity integer NOT NULL DEFAULT 0,
    reorder_threshold integer NOT NULL DEFAULT 0,
    is_active boolean NOT NULL DEFAULT true,
    attr jsonb NOT NULL DEFAULT '{}'::jsonb,
    notes text COLLATE pg_catalog."default",
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_at timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT product_variants_pkey PRIMARY KEY (id),
    CONSTRAINT fk_product_variants_product FOREIGN KEY (product_id)
        REFERENCES public.products (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT fk_product_variants_brand FOREIGN KEY (brand_id)
        REFERENCES public.brands (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT check_stock_quantity_non_negative CHECK (stock_quantity >= 0),
    CONSTRAINT check_reorder_threshold_non_negative CHECK (reorder_threshold >= 0)
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.product_variants OWNER to mohammed;

-- ========================================================
-- 2. Foreign Key Indexes
-- ========================================================

CREATE INDEX IF NOT EXISTS idx_product_variants_product_id
    ON public.product_variants USING btree
    (product_id ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE INDEX IF NOT EXISTS idx_product_variants_brand_id
    ON public.product_variants USING btree
    (brand_id ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

-- ========================================================
-- 3. Unique Indexes
-- ========================================================

CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_sku_lower
    ON public.product_variants USING btree
    (lower(sku::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

-- Fixed: WITH and TABLESPACE clauses placed before WHERE clause
CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_barcode
    ON public.product_variants USING btree
    (barcode COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default
    WHERE barcode IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_name_product_lower
    ON public.product_variants USING btree
    (product_id ASC NULLS LAST, lower(name::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_name_ar_product_lower
    ON public.product_variants USING btree
    (product_id ASC NULLS LAST, lower(name_ar::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

-- ========================================================
-- 4. JSONB & Partial Active Indexes
-- ========================================================

CREATE INDEX IF NOT EXISTS idx_product_variants_attr_gin
    ON public.product_variants USING gin (attr)
    TABLESPACE pg_default;

CREATE INDEX IF NOT EXISTS idx_product_variants_active_product
    ON public.product_variants USING btree
    (product_id ASC NULLS LAST)
    TABLESPACE pg_default
    WHERE is_active = true;

-- ========================================================
-- 5. Trigger
-- ========================================================

DROP TRIGGER IF EXISTS update_product_variants_modtime ON public.product_variants;
CREATE TRIGGER update_product_variants_modtime
    BEFORE UPDATE ON public.product_variants
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();