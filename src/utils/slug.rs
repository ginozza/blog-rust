use deunicode::deunicode;

/// Convierte un texto en un slug URL-friendly
pub fn slugify(text: &str) -> String {
    // Convertir a ASCII (eliminar acentos y caracteres especiales)
    let ascii = deunicode(text);
    
    // Convertir a minúsculas
    let lowercase = ascii.to_lowercase();
    
    // Reemplazar espacios y caracteres no alfanuméricos con guiones
    let mut slug = String::new();
    let mut prev_dash = true; // Para evitar guiones consecutivos
    
    for c in lowercase.chars() {
        if c.is_alphanumeric() {
            slug.push(c);
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }
    
    // Eliminar guiones al inicio y final
    slug = slug.trim_matches('-').to_string();
    
    // Si el slug está vacío, devolver un valor por defecto
    if slug.is_empty() {
        return "post".to_string();
    }
    
    slug
} 