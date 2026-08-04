-- ========================================================
-- 4. Product Variants Table
-- ========================================================
CREATE TABLE IF NOT EXISTS public.product_variants (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    brand_id INT NOT NULL,
    sku VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    stock_quantity INT NOT NULL DEFAULT 0,
    attributes JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_variant_sku UNIQUE (sku),
    CONSTRAINT unique_variant_name UNIQUE (name),
    CONSTRAINT unique_variant_name_ar UNIQUE (name_ar),

    CONSTRAINT fk_product_variants_product 
        FOREIGN KEY (product_id)
        REFERENCES public.products (id)
         ON DELETE RESTRICT 
        ON UPDATE CASCADE,

    CONSTRAINT fk_product_variants_brand 
        FOREIGN KEY (brand_id)
        REFERENCES public.brands (id)
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_product_variants_product_id ON public.product_variants (product_id);
CREATE INDEX IF NOT EXISTS idx_product_variants_brand_id ON public.product_variants (brand_id);

DROP TRIGGER IF EXISTS update_product_variants_modtime ON public.product_variants;
CREATE TRIGGER update_product_variants_modtime
    BEFORE UPDATE ON public.product_variants
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();





