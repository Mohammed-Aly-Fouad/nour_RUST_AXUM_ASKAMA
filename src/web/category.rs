// // مثال توضيحي داخل الـ Handler
// let old_category = get_category_by_id(category_id).await?;

// // مقارنة البيانات الجديدة بالقديمة
// if old_category.name == form.name 
//     && old_category.name_ar == form.name_ar 
//     && old_category.parent_id == form.parent_id 
//     && old_category.notes == form.notes {
    
//     // البيانات مطابقة تماماً! لا داعي لتنفيذ استعلام التحديث في قاعدة البيانات
//     return Ok(Redirect::to("/categories")); // أو إعادة توجيه مباشرة كأن شيئاً لم يكن
// }

// // إذا كانت مختلفة، نقوم بتنفيذ الـ validate أولاً ثم الـ UPDATE في قاعدة البيانات...

// في حالة التعديل ولم يتم تغيير شئ





// لعرف المنتجات

// use askama::Template;
// use axum::response::IntoResponse;

// // تعريف القالب الخاص بعرض الفئات
// #[derive(Template)]
// #[template(path = "categories.html")]
// pub struct CategoriesTemplate {
//     pub categories: Vec<CategoryResponseDto>,
// }

// // دالة الـ Handler في Axum
// async fn list_categories_handler() -> impl IntoResponse {
//     // جلب البيانات من قاعدة البيانات...
//     let categories = vec![]; // مثال
    
//     // إرجاع القالب مباشرة دون الحاجة لتحويله يدوياً لـ HTML
//     CategoriesTemplate { categories }
// }