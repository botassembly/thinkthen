-- Generated internal carrier projections.
package body Thinkthen.Inputs.Native is
function Unwrap (Item : Thinkthen.Inputs.Text) return Requests.T_Authored_name is (Requests.T_Authored_name (Item));
function Wrap (Item : Requests.T_Authored_name) return Thinkthen.Inputs.Text is (Thinkthen.Inputs.Text (Item));
function Unwrap (Item : Thinkthen.Inputs.Text) return Requests.T_Authored_profile is (Requests.T_Authored_profile (Item));
function Wrap (Item : Requests.T_Authored_profile) return Thinkthen.Inputs.Text is (Thinkthen.Inputs.Text (Item));
function Unwrap (Item : Thinkthen.Inputs.Boolean_Value) return Requests.T_Boolean is (Requests.T_Boolean (Item));
function Wrap (Item : Requests.T_Boolean) return Thinkthen.Inputs.Boolean_Value is (Thinkthen.Inputs.Boolean_Value (Item));
function Unwrap (Item : Thinkthen.Inputs.Integer_Value) return Requests.T_Integer is (Requests.T_Integer (Item));
function Wrap (Item : Requests.T_Integer) return Thinkthen.Inputs.Integer_Value is (Thinkthen.Inputs.Integer_Value (Item));
function Unwrap (Item : Thinkthen.Inputs.Value) return Requests.T_JSON is (Requests.T_JSON (Item));
function Wrap (Item : Requests.T_JSON) return Thinkthen.Inputs.Value is (Thinkthen.Inputs.Value (Item));
function Unwrap (Item : Thinkthen.Inputs.Number) return Requests.T_Number is (Requests.T_Number (Item));
function Wrap (Item : Requests.T_Number) return Thinkthen.Inputs.Number is (Thinkthen.Inputs.Number (Item));
function Unwrap (Item : Thinkthen.Inputs.Text) return Requests.T_Text is (Requests.T_Text (Item));
function Wrap (Item : Requests.T_Text) return Thinkthen.Inputs.Text is (Thinkthen.Inputs.Text (Item));
function Unwrap (Item : Thinkthen.Inputs.Unsigned) return Requests.T_Unsigned is (Requests.T_Unsigned (Item));
function Wrap (Item : Requests.T_Unsigned) return Thinkthen.Inputs.Unsigned is (Thinkthen.Inputs.Unsigned (Item));
function Unwrap (Item : Thinkthen.Inputs.Image_Media) return Requests.T_ImageMedia is (Requests.T_ImageMedia'Val (Image_Media'Pos (Item)));
function Wrap (Item : Requests.T_ImageMedia) return Thinkthen.Inputs.Image_Media is (Thinkthen.Inputs.Image_Media'Val (Requests.T_ImageMedia'Pos (Item)));
function Unwrap (Item : Thinkthen.Inputs.Reader_Media) return Requests.T_ReaderMedia is (Requests.T_ReaderMedia'Val (Reader_Media'Pos (Item)));
function Wrap (Item : Requests.T_ReaderMedia) return Thinkthen.Inputs.Reader_Media is (Thinkthen.Inputs.Reader_Media'Val (Requests.T_ReaderMedia'Pos (Item)));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Mode) return Requests.T_RecognitionMode is (Requests.T_RecognitionMode'Val (Recognition_Mode'Pos (Item)));
function Wrap (Item : Requests.T_RecognitionMode) return Thinkthen.Inputs.Recognition_Mode is (Thinkthen.Inputs.Recognition_Mode'Val (Requests.T_RecognitionMode'Pos (Item)));
function Unwrap (Item : Thinkthen.Inputs.Framing) return Requests.T_RequestFraming is (Requests.T_RequestFraming'Val (Framing'Pos (Item)));
function Wrap (Item : Requests.T_RequestFraming) return Thinkthen.Inputs.Framing is (Thinkthen.Inputs.Framing'Val (Requests.T_RequestFraming'Pos (Item)));
function Unwrap (Item : Thinkthen.Inputs.Function_Kind) return Requests.T_RequestFunction is (Requests.T_RequestFunction'Val (Function_Kind'Pos (Item)));
function Wrap (Item : Requests.T_RequestFunction) return Thinkthen.Inputs.Function_Kind is (Thinkthen.Inputs.Function_Kind'Val (Requests.T_RequestFunction'Pos (Item)));
function Unwrap (Item : Thinkthen.Inputs.Source_Unit) return Requests.T_SourceUnit is (Requests.T_SourceUnit'Val (Source_Unit'Pos (Item)));
function Wrap (Item : Requests.T_SourceUnit) return Thinkthen.Inputs.Source_Unit is (Thinkthen.Inputs.Source_Unit'Val (Requests.T_SourceUnit'Pos (Item)));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Definition) return Requests.T_AnnotateDefinition is (Item.Data);
function Wrap (Item : Requests.T_AnnotateDefinition) return Thinkthen.Inputs.Annotate_Definition is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Question) return Requests.T_AnnotateQuestion is (Item.Data);
function Wrap (Item : Requests.T_AnnotateQuestion) return Thinkthen.Inputs.Annotate_Question is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Question_choose) return Requests.T_AnnotateQuestion_choose is (Item.Data);
function Wrap (Item : Requests.T_AnnotateQuestion_choose) return Thinkthen.Inputs.Annotate_Question_choose is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Question_decide) return Requests.T_AnnotateQuestion_decide is (Item.Data);
function Wrap (Item : Requests.T_AnnotateQuestion_decide) return Thinkthen.Inputs.Annotate_Question_decide is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Question_score) return Requests.T_AnnotateQuestion_score is (Item.Data);
function Wrap (Item : Requests.T_AnnotateQuestion_score) return Thinkthen.Inputs.Annotate_Question_score is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Question_tag) return Requests.T_AnnotateQuestion_tag is (Item.Data);
function Wrap (Item : Requests.T_AnnotateQuestion_tag) return Thinkthen.Inputs.Annotate_Question_tag is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.choose) return Requests.T_Authored_choose is (Item.Data);
function Wrap (Item : Requests.T_Authored_choose) return Thinkthen.Inputs.choose is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.choose_batch) return Requests.T_Authored_choose_field_batch is (Item.Data);
function Wrap (Item : Requests.T_Authored_choose_field_batch) return Thinkthen.Inputs.choose_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.criterion) return Requests.T_Authored_criterion is (Item.Data);
function Wrap (Item : Requests.T_Authored_criterion) return Thinkthen.Inputs.criterion is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.cut) return Requests.T_Authored_cut is (Item.Data);
function Wrap (Item : Requests.T_Authored_cut) return Thinkthen.Inputs.cut is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.decide) return Requests.T_Authored_decide is (Item.Data);
function Wrap (Item : Requests.T_Authored_decide) return Thinkthen.Inputs.decide is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.decide_batch) return Requests.T_Authored_decide_field_batch is (Item.Data);
function Wrap (Item : Requests.T_Authored_decide_field_batch) return Thinkthen.Inputs.decide_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.description) return Requests.T_Authored_description is (Item.Data);
function Wrap (Item : Requests.T_Authored_description) return Thinkthen.Inputs.description is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.find) return Requests.T_Authored_find is (Item.Data);
function Wrap (Item : Requests.T_Authored_find) return Thinkthen.Inputs.find is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Declaration) return Requests.T_Authored_inputDeclaration is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputDeclaration) return Thinkthen.Inputs.input_Declaration is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Declaration_object) return Requests.T_Authored_inputDeclaration_object is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputDeclaration_object) return Thinkthen.Inputs.input_Declaration_object is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Declaration_string) return Requests.T_Authored_inputDeclaration_string is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputDeclaration_string) return Thinkthen.Inputs.input_Declaration_string is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Property) return Requests.T_Authored_inputProperty is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputProperty) return Thinkthen.Inputs.input_Property is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Property_array) return Requests.T_Authored_inputProperty_array is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputProperty_array) return Thinkthen.Inputs.input_Property_array is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Array_Item_Type) return Requests.T_Authored_inputProperty_array_field_items is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputProperty_array_field_items) return Thinkthen.Inputs.Array_Item_Type is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Property_boolean) return Requests.T_Authored_inputProperty_boolean is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputProperty_boolean) return Thinkthen.Inputs.input_Property_boolean is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Property_number) return Requests.T_Authored_inputProperty_number is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputProperty_number) return Thinkthen.Inputs.input_Property_number is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.input_Property_string) return Requests.T_Authored_inputProperty_string is (Item.Data);
function Wrap (Item : Requests.T_Authored_inputProperty_string) return Thinkthen.Inputs.input_Property_string is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Tag_Labels) return Requests.T_Authored_labels is (Item.Data);
function Wrap (Item : Requests.T_Authored_labels) return Thinkthen.Inputs.Tag_Labels is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Score_Levels) return Requests.T_Authored_levels is (Item.Data);
function Wrap (Item : Requests.T_Authored_levels) return Thinkthen.Inputs.Score_Levels is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Choice_Options) return Requests.T_Authored_options is (Item.Data);
function Wrap (Item : Requests.T_Authored_options) return Thinkthen.Inputs.Choice_Options is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.pointers) return Requests.T_Authored_pointers is (Item.Data);
function Wrap (Item : Requests.T_Authored_pointers) return Thinkthen.Inputs.pointers is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question_Wording) return Requests.T_Authored_questionText is (Item.Data);
function Wrap (Item : Requests.T_Authored_questionText) return Thinkthen.Inputs.Question_Wording is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.relate) return Requests.T_Authored_relate is (Item.Data);
function Wrap (Item : Requests.T_Authored_relate) return Thinkthen.Inputs.relate is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.relation) return Requests.T_Authored_relation is (Item.Data);
function Wrap (Item : Requests.T_Authored_relation) return Thinkthen.Inputs.relation is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.score) return Requests.T_Authored_score is (Item.Data);
function Wrap (Item : Requests.T_Authored_score) return Thinkthen.Inputs.score is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.score_batch) return Requests.T_Authored_score_field_batch is (Item.Data);
function Wrap (Item : Requests.T_Authored_score_field_batch) return Thinkthen.Inputs.score_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.tag) return Requests.T_Authored_tag is (Item.Data);
function Wrap (Item : Requests.T_Authored_tag) return Thinkthen.Inputs.tag is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.tag_batch) return Requests.T_Authored_tag_field_batch is (Item.Data);
function Wrap (Item : Requests.T_Authored_tag_field_batch) return Thinkthen.Inputs.tag_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Probability_Band) return Requests.T_Authored_threshold is (Item.Data);
function Wrap (Item : Requests.T_Authored_threshold) return Thinkthen.Inputs.Probability_Band is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Context_Schema) return Requests.T_ContextSchema is (Item.Data);
function Wrap (Item : Requests.T_ContextSchema) return Thinkthen.Inputs.Context_Schema is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Option_Schema) return Requests.T_OptionSchema is (Item.Data);
function Wrap (Item : Requests.T_OptionSchema) return Thinkthen.Inputs.Option_Schema is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Example) return Requests.T_RecognitionExample is (Item.Data);
function Wrap (Item : Requests.T_RecognitionExample) return Thinkthen.Inputs.Recognition_Example is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Example_Entity) return Requests.T_RecognitionExampleEntity is (Item.Data);
function Wrap (Item : Requests.T_RecognitionExampleEntity) return Thinkthen.Inputs.Recognition_Example_Entity is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Example_Text) return Requests.T_RecognitionExampleText is (Item.Data);
function Wrap (Item : Requests.T_RecognitionExampleText) return Thinkthen.Inputs.Recognition_Example_Text is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Seed_Span) return Requests.T_RecognitionSeedSpan is (Item.Data);
function Wrap (Item : Requests.T_RecognitionSeedSpan) return Thinkthen.Inputs.Recognition_Seed_Span is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Requests.T_RecognitionStageContext is (Item.Data);
function Wrap (Item : Requests.T_RecognitionStageContext) return Thinkthen.Inputs.Recognition_Stage_Context is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognize_Definition) return Requests.T_RecognizeDefinition is (Item.Data);
function Wrap (Item : Requests.T_RecognizeDefinition) return Thinkthen.Inputs.Recognize_Definition is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Recognition_Rules) return Requests.T_RecognizeQuestion is (Item.Data);
function Wrap (Item : Requests.T_RecognizeQuestion) return Thinkthen.Inputs.Recognition_Rules is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Relate_Fields) return Requests.T_RelateFields is (Item.Data);
function Wrap (Item : Requests.T_RelateFields) return Thinkthen.Inputs.Relate_Fields is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Relation_Rules) return Requests.T_RelateQuestion is (Item.Data);
function Wrap (Item : Requests.T_RelateQuestion) return Thinkthen.Inputs.Relation_Rules is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Batch) return Requests.T_RequestBatch is (Item.Data);
function Wrap (Item : Requests.T_RequestBatch) return Thinkthen.Inputs.Batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition) return Requests.T_RequestDefinition is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition) return Thinkthen.Inputs.Definition is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Requests.T_RequestDefinition_AnnotateDefinition is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_AnnotateDefinition) return Thinkthen.Inputs.Definition_Annotate_Definition is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_choose) return Requests.T_RequestDefinition_Authored_choose is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_choose) return Thinkthen.Inputs.Definition_choose is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_choose_batch) return Requests.T_RequestDefinition_Authored_choose_field_batch is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_choose_field_batch) return Thinkthen.Inputs.Definition_choose_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_decide) return Requests.T_RequestDefinition_Authored_decide is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_decide) return Thinkthen.Inputs.Definition_decide is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_decide_batch) return Requests.T_RequestDefinition_Authored_decide_field_batch is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_decide_field_batch) return Thinkthen.Inputs.Definition_decide_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_find) return Requests.T_RequestDefinition_Authored_find is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_find) return Thinkthen.Inputs.Definition_find is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_relate) return Requests.T_RequestDefinition_Authored_relate is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_relate) return Thinkthen.Inputs.Definition_relate is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_score) return Requests.T_RequestDefinition_Authored_score is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_score) return Thinkthen.Inputs.Definition_score is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_score_batch) return Requests.T_RequestDefinition_Authored_score_field_batch is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_score_field_batch) return Thinkthen.Inputs.Definition_score_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_tag) return Requests.T_RequestDefinition_Authored_tag is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_tag) return Thinkthen.Inputs.Definition_tag is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_tag_batch) return Requests.T_RequestDefinition_Authored_tag_field_batch is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_Authored_tag_field_batch) return Thinkthen.Inputs.Definition_tag_batch is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Requests.T_RequestDefinition_RecognizeDefinition is (Item.Data);
function Wrap (Item : Requests.T_RequestDefinition_RecognizeDefinition) return Thinkthen.Inputs.Definition_Recognize_Definition is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Image) return Requests.T_RequestImage is (Item.Data);
function Wrap (Item : Requests.T_RequestImage) return Thinkthen.Inputs.Image is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Image_bytes) return Requests.T_RequestImage_bytes is (Item.Data);
function Wrap (Item : Requests.T_RequestImage_bytes) return Thinkthen.Inputs.Image_bytes is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Image_file) return Requests.T_RequestImage_file is (Item.Data);
function Wrap (Item : Requests.T_RequestImage_file) return Thinkthen.Inputs.Image_file is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input) return Requests.T_RequestInput is (Item.Data);
function Wrap (Item : Requests.T_RequestInput) return Thinkthen.Inputs.Input is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_entities) return Requests.T_RequestInput_entities is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_entities) return Thinkthen.Inputs.Input_entities is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_feed) return Requests.T_RequestInput_feed is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_feed) return Thinkthen.Inputs.Input_feed is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_json) return Requests.T_RequestInput_json is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_json) return Thinkthen.Inputs.Input_json is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_records) return Requests.T_RequestInput_records is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_records) return Thinkthen.Inputs.Input_records is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_source) return Requests.T_RequestInput_source is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_source) return Thinkthen.Inputs.Input_source is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_text) return Requests.T_RequestInput_text is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_text) return Thinkthen.Inputs.Input_text is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Input_units) return Requests.T_RequestInput_units is (Item.Data);
function Wrap (Item : Requests.T_RequestInput_units) return Thinkthen.Inputs.Input_units is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Item) return Requests.T_RequestItem is (Item.Data);
function Wrap (Item : Requests.T_RequestItem) return Thinkthen.Inputs.Item is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Options) return Requests.T_RequestOptions is (Item.Data);
function Wrap (Item : Requests.T_RequestOptions) return Thinkthen.Inputs.Options is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Original) return Requests.T_RequestOriginal is (Item.Data);
function Wrap (Item : Requests.T_RequestOriginal) return Thinkthen.Inputs.Original is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Original_json) return Requests.T_RequestOriginal_json is (Item.Data);
function Wrap (Item : Requests.T_RequestOriginal_json) return Thinkthen.Inputs.Original_json is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Original_text) return Requests.T_RequestOriginal_text is (Item.Data);
function Wrap (Item : Requests.T_RequestOriginal_text) return Thinkthen.Inputs.Original_text is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question) return Requests.T_RequestQuestion is (Item.Data);
function Wrap (Item : Requests.T_RequestQuestion) return Thinkthen.Inputs.Question is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question_definition) return Requests.T_RequestQuestion_definition is (Item.Data);
function Wrap (Item : Requests.T_RequestQuestion_definition) return Thinkthen.Inputs.Question_definition is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question_file) return Requests.T_RequestQuestion_file is (Item.Data);
function Wrap (Item : Requests.T_RequestQuestion_file) return Thinkthen.Inputs.Question_file is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question_name) return Requests.T_RequestQuestion_name is (Item.Data);
function Wrap (Item : Requests.T_RequestQuestion_name) return Thinkthen.Inputs.Question_name is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question_reference) return Requests.T_RequestQuestion_reference is (Item.Data);
function Wrap (Item : Requests.T_RequestQuestion_reference) return Thinkthen.Inputs.Question_reference is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Question_text) return Requests.T_RequestQuestion_text is (Item.Data);
function Wrap (Item : Requests.T_RequestQuestion_text) return Thinkthen.Inputs.Question_text is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Reader) return Requests.T_RequestReader is (Item.Data);
function Wrap (Item : Requests.T_RequestReader) return Thinkthen.Inputs.Reader is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Reader_Error) return Requests.T_RequestReaderFailure is (Item.Data);
function Wrap (Item : Requests.T_RequestReaderFailure) return Thinkthen.Inputs.Reader_Error is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Reader_Failure_invalid_input) return Requests.T_RequestReaderFailure_invalid_input is (Item.Data);
function Wrap (Item : Requests.T_RequestReaderFailure_invalid_input) return Thinkthen.Inputs.Reader_Failure_invalid_input is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Reader_Failure_io) return Requests.T_RequestReaderFailure_io is (Item.Data);
function Wrap (Item : Requests.T_RequestReaderFailure_io) return Thinkthen.Inputs.Reader_Failure_io is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Reader_Failure_producer) return Requests.T_RequestReaderFailure_producer is (Item.Data);
function Wrap (Item : Requests.T_RequestReaderFailure_producer) return Thinkthen.Inputs.Reader_Failure_producer is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Reader_Failure_utf8) return Requests.T_RequestReaderFailure_utf8 is (Item.Data);
function Wrap (Item : Requests.T_RequestReaderFailure_utf8) return Thinkthen.Inputs.Reader_Failure_utf8 is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Feed_Item) return Requests.T_RequestSessionDescriptor is (Item.Data);
function Wrap (Item : Requests.T_RequestSessionDescriptor) return Thinkthen.Inputs.Feed_Item is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Source) return Requests.T_RequestSource is (Item.Data);
function Wrap (Item : Requests.T_RequestSource) return Thinkthen.Inputs.Source is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Threshold) return Requests.T_RequestThreshold is (Item.Data);
function Wrap (Item : Requests.T_RequestThreshold) return Thinkthen.Inputs.Threshold is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Session_Source_Location) return Requests.T_SessionSourceLocation is (Item.Data);
function Wrap (Item : Requests.T_SessionSourceLocation) return Thinkthen.Inputs.Session_Source_Location is ((Data => Item));
function Unwrap (Item : Thinkthen.Inputs.Annotate_Question_Map) return Requests.T_AnnotateQuestion_Map is
Result : Requests.T_AnnotateQuestion_Map;
begin
for Child of Item loop
Result.Append (Requests.T_AnnotateQuestion_Map_Entry'(Key => Child.Name, Value => Unwrap (Child.Item)));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_AnnotateQuestion_Map) return Thinkthen.Inputs.Annotate_Question_Map is
Result : Annotate_Question_Map (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := (Name => Child.Key, Item => Wrap (Child.Value));
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.criterion_Map) return Requests.T_Authored_criterion_Map is
Result : Requests.T_Authored_criterion_Map;
begin
for Child of Item loop
Result.Append (Requests.T_Authored_criterion_Map_Entry'(Key => Child.Name, Value => Unwrap (Child.Item)));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_Authored_criterion_Map) return Thinkthen.Inputs.criterion_Map is
Result : criterion_Map (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := (Name => Child.Key, Item => Wrap (Child.Value));
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.description_Map) return Requests.T_Authored_description_Map is
Result : Requests.T_Authored_description_Map;
begin
for Child of Item loop
Result.Append (Requests.T_Authored_description_Map_Entry'(Key => Child.Name, Value => Unwrap (Child.Item)));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_Authored_description_Map) return Thinkthen.Inputs.description_Map is
Result : description_Map (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := (Name => Child.Key, Item => Wrap (Child.Value));
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.input_Property_Map) return Requests.T_Authored_inputProperty_Map is
Result : Requests.T_Authored_inputProperty_Map;
begin
for Child of Item loop
Result.Append (Requests.T_Authored_inputProperty_Map_Entry'(Key => Child.Name, Value => Unwrap (Child.Item)));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_Authored_inputProperty_Map) return Thinkthen.Inputs.input_Property_Map is
Result : input_Property_Map (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := (Name => Child.Key, Item => Wrap (Child.Value));
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.name_List) return Requests.T_Authored_name_List is
Result : Requests.T_Authored_name_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_Authored_name_List) return Thinkthen.Inputs.name_List is
Result : name_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.relation_List) return Requests.T_Authored_relation_List is
Result : Requests.T_Authored_relation_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_Authored_relation_List) return Thinkthen.Inputs.relation_List is
Result : relation_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.JSON_List) return Requests.T_JSON_List is
Result : Requests.T_JSON_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_JSON_List) return Thinkthen.Inputs.JSON_List is
Result : JSON_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.JSON_Map) return Requests.T_JSON_Map is
Result : Requests.T_JSON_Map;
begin
for Child of Item loop
Result.Append (Requests.T_JSON_Map_Entry'(Key => Child.Name, Value => Unwrap (Child.Item)));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_JSON_Map) return Thinkthen.Inputs.JSON_Map is
Result : JSON_Map (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := (Name => Child.Key, Item => Wrap (Child.Value));
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Option_Schema_List) return Requests.T_OptionSchema_List is
Result : Requests.T_OptionSchema_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_OptionSchema_List) return Thinkthen.Inputs.Option_Schema_List is
Result : Option_Schema_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Recognition_Example_Entity_List) return Requests.T_RecognitionExampleEntity_List is
Result : Requests.T_RecognitionExampleEntity_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_RecognitionExampleEntity_List) return Thinkthen.Inputs.Recognition_Example_Entity_List is
Result : Recognition_Example_Entity_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Recognition_Example_List) return Requests.T_RecognitionExample_List is
Result : Requests.T_RecognitionExample_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_RecognitionExample_List) return Thinkthen.Inputs.Recognition_Example_List is
Result : Recognition_Example_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Recognition_Seed_Span_List) return Requests.T_RecognitionSeedSpan_List is
Result : Requests.T_RecognitionSeedSpan_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_RecognitionSeedSpan_List) return Thinkthen.Inputs.Recognition_Seed_Span_List is
Result : Recognition_Seed_Span_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Image_List) return Requests.T_RequestImage_List is
Result : Requests.T_RequestImage_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_RequestImage_List) return Thinkthen.Inputs.Image_List is
Result : Image_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Item_List) return Requests.T_RequestItem_List is
Result : Requests.T_RequestItem_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_RequestItem_List) return Thinkthen.Inputs.Item_List is
Result : Item_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Thinkthen.Inputs.Text_List) return Requests.T_Text_List is
Result : Requests.T_Text_List;
begin
for Child of Item loop
Result.Append (Unwrap (Child));
end loop;
return Result;
end Unwrap;
function Wrap (Item : Requests.T_Text_List) return Thinkthen.Inputs.Text_List is
Result : Text_List (1 .. Natural (Item.Length));
Position : Positive := 1;
begin
for Child of Item loop
Result (Position) := Wrap (Child);
Position := Position + 1;
end loop;
return Result;
end Wrap;
function Unwrap (Item : Call_Options) return Requests.T_RequestOptions_Optional is (Item.Data);
function Wrap (Item : Requests.T_RequestOptions_Optional) return Call_Options is ((Data => Item));
end Thinkthen.Inputs.Native;
