Sometimes it will say

```
Caused by:
    In Device::create_render_pipeline
      note: label = `post_process_pipeline`
    Error matching ShaderStages(FRAGMENT) shader requirements against the pipeline
    Shader global ResourceBinding { group: 0, binding: 31 } is not available in the layout pipeline layout
    Binding is missing from the pipeline layout
```

Other times it will say 

```
Caused by:
    In Device::create_render_pipeline
      note: label = `post_process_pipeline`
    Error matching ShaderStages(FRAGMENT) shader requirements against the pipeline
    Shader global ResourceBinding { group: 0, binding: 11 } is not available in the layout pipeline layout
    Binding is missing from the pipeline layout
```

The issue still occurs of the both pipelines share the same `post_processing.wgsl`. 



If the screen_texture binding:
```wgsl
@group(#{TARGET_GROUP_N}) @binding(#{TARGET_ENTRY_N}) var screen_texture: texture_2d<f32>;
``` 
is moved out of common and into `post_processing_1.wgsl` or `post_processing_2.wgsl` (or a shared `post_processing.wgsl` file) it works as expected.