use std::{collections::HashMap, fs, path::Path};

use memvdb::{CacheDB, Distance, Embedding};
use candle_core::{DType, Device, Tensor};
use candle_nn::Module;
use prefstore::getcustom;
use shiva::core::{bytes::Bytes, Element, TransformerTrait};
use anyhow::anyhow;
use text_splitter::TextSplitter;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

// Candle-based embedding helper struct for Lilim app integration
pub struct CandleEmbedder {
    device: Device,
}

impl CandleEmbedder {
    pub fn new() -> anyhow::Result<Self> {
        let device = Device::cpu();
        Ok(Self { device })
    }
    
    pub async fn generate_embeddings(&self, texts: Vec<String>) -> anyhow::Result<Vec<Vec<f32>>> {
        // Placeholder implementation for candle-based embeddings
        // In a real implementation with Lilim app, this would:
        // 1. Connect to the Lilim candle service
        // 2. Send embedding requests
        // 3. Receive embedding vectors
        
        // For now, return mock embeddings with the correct dimension (768 for nomic-embed-text compatibility)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut embeddings = Vec::new();
        for text in texts {
            let mut embedding = vec![0.0f32; 768];
            // Create a deterministic but varied embedding based on text content
            let mut hasher = DefaultHasher::new();
            text.hash(&mut hasher);
            let hash = hasher.finish();
            
            // Use the hash to create varied values
            for (i, val) in embedding.iter_mut().enumerate() {
                *val = ((hash as usize + i) % 1000) as f32 / 1000.0;
            }
            
            // Normalize to some extent
            let sum: f32 = embedding.iter().sum();
            if sum > 0.0 {
                for val in &mut embedding {
                    *val /= sum;
                }
            }
            
            embeddings.push(embedding);
        }
        Ok(embeddings)
    }
    
    pub async fn generate_text_stream(&self, prompt: String) -> anyhow::Result<Vec<String>> {
        // Placeholder for text generation with candle
        // This would use the candle model for text generation via Lilim app
        // For now, return a mock stream response
        let responses = vec![
            "I am a helpful AI assistant integrated with Reliquary file manager.".to_string(),
            " I can help you with your files and answer questions about them.".to_string(),
        ];
        Ok(responses)
    }
}
pub struct ExtractedDocument {
    pub content: String,
    pub metadata: HashMap<String, String>,
}
fn collect_text_from_elements(elements: &Vec<&Element>, collected_text: &mut String) {
    for element in elements {
       match element {
           Element::Text{text,size} => {
               collected_text.push_str(&text);
           }
           Element::Paragraph{elements} => {
               for items in elements.iter(){
                   collect_text_from_elements(&vec![items], collected_text);
               }
               collected_text.push_str("\n\n");
           }
           Element::Header{text,level} => {
               collected_text.push_str(&text);
               collected_text.push(' ');
           }
           Element::List{elements,..} => {
               for (i, item) in elements.iter().enumerate() {
                   collect_text_from_elements(&vec![&item.element], collected_text);
                   collected_text.push('\n');
               }
               collected_text.push('\n');
           }
           Element::Table { headers, rows } => {
               for row in rows {
                   for cell in &row.cells {
                       collect_text_from_elements(&vec![&cell.element], collected_text);
                       collected_text.push('\t');
                   }
                   collected_text.push('\n');
               }
               collected_text.push('\n');
           }
           Element::Image(img) => {
               collected_text.push_str(&format!("[Image: {}]", img.alt()));
               collected_text.push(' ');
           }
           Element::Hyperlink { title, url, alt, size }=>{
               collected_text.push_str(&format!("{}", title));
               collected_text.push_str(&format!(" ({})", url));
               collected_text.push(' ');
           }
       }
   }
}
fn get_document_type(path: &Path) -> Option<&'static str> {
   path.extension().and_then(|ext| ext.to_str()).map(|s| match s {
       "txt" => "text",
       "rs" => "text",
       "md" => "markdown",
       "html" | "htm" => "html",
       "pdf" => "pdf",
       "json" => "json",
       "csv" => "csv",
       "rtf" => "rtf",
       "docx" => "docx",
       "xml" => "xml",
       "xls" => "xls",
       "xlsx" => "xlsx",
       "ods" => "ods",
       "typst" => "typst",
       _ => "unknown",
   })
}

pub  fn load_document_and_extract_text(file_path: &Path) -> anyhow::Result<ExtractedDocument> {
let file_bytes = fs::read(file_path)?;
let input_bytes = Bytes::from(file_bytes);

let doc_type = get_document_type(file_path)
   .ok_or_else(|| anyhow!("Could not determine document type for {:?}", file_path))?;

let document: shiva::core::Document = match doc_type {
   "text" => shiva::text::Transformer::parse(&input_bytes)?,
   "markdown" => shiva::markdown::Transformer::parse(&input_bytes)?,
   "html" => shiva::html::Transformer::parse(&input_bytes)?,
   "pdf" => shiva::pdf::Transformer::parse(&input_bytes)?,
   "json" => shiva::json::Transformer::parse(&input_bytes)?,
   "csv" => shiva::csv::Transformer::parse(&input_bytes)?,
   "rtf" => shiva::rtf::Transformer::parse(&input_bytes)?,
   "docx" => shiva::docx::Transformer::parse(&input_bytes)?,
   "xml" => shiva::xml::Transformer::parse(&input_bytes)?,
   "xls" => shiva::xls::Transformer::parse(&input_bytes)?,
   "xlsx" => shiva::xlsx::Transformer::parse(&input_bytes)?,
   "ods" => shiva::ods::Transformer::parse(&input_bytes)?,
   "typst" => shiva::typst::Transformer::parse(&input_bytes)?,
   _ => return Err(anyhow!("Unsupported document type: {}", doc_type)),
};

let mut collected_text = String::new();
collect_text_from_elements(&document.get_all_elements(), &mut collected_text);

let mut metadata = HashMap::new();
metadata.insert("file_name".to_string(), file_path.file_name().unwrap_or_default().to_string_lossy().into_owned());
metadata.insert("file_path".to_string(), file_path.to_string_lossy().into_owned());

Ok(ExtractedDocument {
   content: collected_text.trim().to_string(),
   metadata,
})
}


#[tokio::test]
async fn embedtest() {
    use std::collections::HashMap;
    use std::path::Path;

    let question = "hi".to_string();
    let path = "C:\\Users\\wkramer\\DeclarationandAuthorization_FILLED.pdf".to_string();

    println!("Path {} exists? {}", path, Path::new(&path).exists());

    // Initialize candle embedder for Lilim integration
    let candle_embedder = CandleEmbedder::new().unwrap();

    // Load and chunk document
    let input_vec = load_document_and_extract_text(Path::new(&path)).unwrap();
    let splitter = TextSplitter::new(256);
    let mut seen = std::collections::HashSet::new();
    let chunks: Vec<&str> = splitter.chunks(&input_vec.content).filter(|c| seen.insert(*c)).collect();
    let chunk_strings: Vec<String> = chunks.clone().map(|s| s.to_string()).collect();

    // Generate embeddings using candle (via Lilim app)
    match candle_embedder.generate_embeddings(chunk_strings).await {
        Ok(embeddings) => {
            println!("Successfully generated {} embeddings.", embeddings.len());

            // Store in vector database
            let mut db = CacheDB::new();
            db.create_collection(path.clone(), 768, Distance::Cosine).unwrap();

            for (i, embedding) in embeddings.iter().enumerate() {
                let embedding_data = Embedding {
                    id: HashMap::from([(format!("title"), chunks[i].to_string())]),
                    vector: embedding.clone(),
                    metadata: Some(input_vec.metadata.clone()),
                };

                db.insert_into_collection(&path, embedding_data).unwrap();
            }

            // Generate embeddings for the question
            let mut seen = std::collections::HashSet::new();
            let question_chunks: Vec<&str> = splitter.chunks(&question).filter(|c| seen.insert(*c)).collect();
            let question_strings: Vec<String> = question_chunks.clone().map(|s| s.to_string()).collect();

            if let Ok(query_embeddings) = candle_embedder.generate_embeddings(question_strings).await {
                let collection = db.get_collection(&path).unwrap();
                let mut retrieved_context = String::new();

                for embedding in &query_embeddings {
                    for result in collection.get_similarity(embedding, 2) {
                        if let Some(title) = result.embedding.id.get(&format!("title")) {
                            retrieved_context.push_str(title);
                            retrieved_context.push_str("\n");
                        }
                    }
                }

                println!("Retrieved context:\n{}", retrieved_context);

                let prompt = format!("Given the following context which are contents of a file, answer the question accurately and concisely. If the answer is not in the context, state that you cannot answer from the provided information.\n\nContext: {}\n\nQuestion: {}", retrieved_context.trim(), question);

                // Use candle for text generation
                if let Ok(responses) = candle_embedder.generate_text_stream(prompt).await {
                    println!("\n--- LLM Response ---");
                    for response in responses {
                        println!("{}", response);
                    }
                    println!("--------------------");
                }
            }
        }
        Err(e) => {
            println!("Failed to generate embeddings: {}", e);
        }
    }
}
