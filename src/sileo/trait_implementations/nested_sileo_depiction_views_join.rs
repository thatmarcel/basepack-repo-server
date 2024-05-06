use crate::sileo::sileo_depiction_view::SileoDepictionView;

pub trait NestedSileoDepictionViewsJoin {
    fn join_with(self, separator_views: Vec<Box<dyn SileoDepictionView>>) -> Vec<Box<dyn SileoDepictionView>>;
}

impl NestedSileoDepictionViewsJoin for Vec<Vec<Box<dyn SileoDepictionView>>> {
    fn join_with(self, separator_views: Vec<Box<dyn SileoDepictionView>>) -> Vec<Box<dyn SileoDepictionView>> {
        let mut result: Vec<Box<dyn SileoDepictionView>> = Vec::new();
        
        for mut views in self {
            if !result.is_empty() {
                result.append(&mut separator_views.iter().cloned().collect());
            }
            
            result.append(&mut views);
        }
        
        result
    }
}