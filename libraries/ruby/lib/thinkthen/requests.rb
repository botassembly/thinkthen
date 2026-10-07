# frozen_string_literal: true
require_relative "complete"
module ThinkThen
  module Complete
    Request = Struct.new(:function, :question, :input, :controls, keyword_init: true) do
      def inspect = "<request: content withheld>"
      def question_json
        raise ArgumentError, "question files require the native loader" if question.is_a?(QuestionFile)
        Complete.to_json_value(question)
      end
    end
    EXPECTED = {
      "decide" => [DecideSpec, QuestionFile], "choose" => [ChooseSpec, QuestionFile],
      "tag" => [TagSpec, QuestionFile], "score" => [ScoreSpec, QuestionFile],
      "filter" => [DecideSpec, QuestionFile], "rank" => [DecideSpec, ScoreSpec, QuestionSet, QuestionFile],
      "find" => [FindSpec, QuestionFile], "annotate" => [QuestionSet, QuestionFile],
      "recognize" => [RecognitionSpec, QuestionFile], "relate" => [RelationSpec, QuestionFile]
    }.freeze
    def self.build(verb, question, input, controls)
      raise ArgumentError, "wrong question kind" unless EXPECTED.fetch(verb).any? { |t| question.is_a?(t) }
      raise ArgumentError, "explicit input required" unless [TextInput, RecordInput, CandidateInput, ImageInput, Files].any? { |t| input.is_a?(t) }
      question = decode(question.class.name.split("::").last, to_json_value(question))
      input = decode(input.class.name.split("::").last, to_json_value(input))
      controls = decode("Controls", controls.nil? ? {} : to_json_value(controls))
      images = input.is_a?(ImageInput) || (input.is_a?(Files) && input.media == "image")
      raise ArgumentError, "this function is text-only" if images && !%w[decide choose score].include?(verb)
      raise ArgumentError, "candidates require find" if input.is_a?(CandidateInput) && verb != "find"
      if input.is_a?(TextInput) && %w[filter rank find annotate relate].include?(verb)
        raise ArgumentError, "this function requires a complete record set"
      end
      if input.is_a?(Files)
        raise ArgumentError, "window requires window units and a size" unless (input.unit == "window") == !input.window.equal?(ABSENT)
        raise ArgumentError, "image sources require file units" if input.media == "image" && input.unit != "file"
      end
      raise ArgumentError, "images require attachments" if input.is_a?(ImageInput) && input.images.empty?
      raise ArgumentError, "top requires rank" unless controls.top.equal?(ABSENT) || verb == "rank"
      raise ArgumentError, "none requires find" unless controls.none.equal?(ABSENT) || verb == "find"
      Request.new(function: verb.freeze, question: question, input: input, controls: controls).freeze
    end
    def self.decide(question, input, controls = nil) = build("decide", question, input, controls)
    def self.choose(question, input, controls = nil) = build("choose", question, input, controls)
    def self.tag(question, input, controls = nil) = build("tag", question, input, controls)
    def self.score(question, input, controls = nil) = build("score", question, input, controls)
    def self.filter(question, input, controls = nil) = build("filter", question, input, controls)
    def self.rank(question, input, controls = nil) = build("rank", question, input, controls)
    def self.find(question, input, controls = nil) = build("find", question, input, controls)
    def self.annotate(question, input, controls = nil) = build("annotate", question, input, controls)
    def self.recognize(question, input, controls = nil) = build("recognize", question, input, controls)
    def self.relate(question, input, controls = nil) = build("relate", question, input, controls)
  end
end
