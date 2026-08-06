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

    -- Data Integrity Constraints
    CONSTRAINT chk_stock_quantity_non_negative CHECK (stock_quantity >= 0),

    -- Foreign Key Constraints
    CONSTRAINT fk_product_variants_product 
        FOREIGN KEY (product_id)
        REFERENCES public.products (id)
        ON DELETE RESTRICT 
        ON UPDATE CASCADE,

    CONSTRAINT fk_product_variants_brand 
        FOREIGN KEY (brand_id)
        REFERENCES public.brands (id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);

-- Foreign key lookup indexes
CREATE INDEX IF NOT EXISTS idx_product_variants_product_id 
    ON public.product_variants (product_id);

CREATE INDEX IF NOT EXISTS idx_product_variants_brand_id 
    ON public.product_variants (brand_id);

-- JSONB index for fast attribute querying (e.g. searching by color, size)
CREATE INDEX IF NOT EXISTS idx_product_variants_attributes 
    ON public.product_variants USING GIN (attributes);

-- Case-Insensitive Unique Indexes
-- Enforces uniqueness for SKU, name, and name_ar regardless of letter casing
CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_sku_lower 
    ON public.product_variants (LOWER(sku));

CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_name_lower 
    ON public.product_variants (LOWER(name));

CREATE UNIQUE INDEX IF NOT EXISTS idx_product_variants_unique_name_ar_lower 
    ON public.product_variants (LOWER(name_ar));

-- Updated-at trigger
DROP TRIGGER IF EXISTS update_product_variants_modtime ON public.product_variants;
CREATE TRIGGER update_product_variants_modtime
    BEFORE UPDATE ON public.product_variants
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();