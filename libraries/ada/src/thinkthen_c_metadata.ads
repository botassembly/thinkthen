-- Counted native carriers. Typed data never uses a JSON buffer.
with Interfaces;
with Interfaces.C;
with System;
with Thinkthen_C_Inputs; use Thinkthen_C_Inputs;
package Thinkthen_C_Metadata is
   C_ORIGIN_LIVE_V1 : constant Interfaces.Unsigned_32 := 1;
   C_ORIGIN_CACHE_V1 : constant Interfaces.Unsigned_32 := 2;
   C_ORIGIN_REPLAY_V1 : constant Interfaces.Unsigned_32 := 3;
   C_ORIGIN_PROXY_V1 : constant Interfaces.Unsigned_32 := 4;
   C_ORIGIN_MEMORY_V1 : constant Interfaces.Unsigned_32 := 5;
   C_ID_OBSERVATION_V1 : constant Interfaces.Unsigned_32 := 1;
   C_ID_FAILURE_V1 : constant Interfaces.Unsigned_32 := 2;
   C_BATCH_RECORDS_V1 : constant Interfaces.Unsigned_32 := 1;
   C_BATCH_MAX_V1 : constant Interfaces.Unsigned_32 := 2;
   C_ATTEMPT_OK_V1 : constant Interfaces.Unsigned_32 := 1;
   C_ATTEMPT_STATUS_V1 : constant Interfaces.Unsigned_32 := 2;
   C_ATTEMPT_TRANSPORT_V1 : constant Interfaces.Unsigned_32 := 3;
   C_STOP_USAGE_V1 : constant Interfaces.Unsigned_32 := 1;
   C_STOP_LOCAL_V1 : constant Interfaces.Unsigned_32 := 2;
   C_STOP_NO_KEY_V1 : constant Interfaces.Unsigned_32 := 3;
   C_STOP_TRANSPORT_V1 : constant Interfaces.Unsigned_32 := 4;
   C_STOP_STATUS_V1 : constant Interfaces.Unsigned_32 := 5;
   C_STOP_TOO_LARGE_V1 : constant Interfaces.Unsigned_32 := 6;
   C_STOP_REPLY_V1 : constant Interfaces.Unsigned_32 := 7;
   C_STOP_BACKEND_V1 : constant Interfaces.Unsigned_32 := 8;
   C_STOP_CANCELLED_V1 : constant Interfaces.Unsigned_32 := 9;
   C_STOP_DEFECT_V1 : constant Interfaces.Unsigned_32 := 10;
   C_STOP_DEADLINE_V1 : constant Interfaces.Unsigned_32 := 11;
   type Usage_V1 is record
      Input_Tokens : Interfaces.Unsigned_64 := 0;
      Output_Tokens : Interfaces.Unsigned_64 := 0;
   end record with Convention => C;
   for Usage_V1 use record
      Input_Tokens at 0 range 0 .. 63;
      Output_Tokens at 8 range 0 .. 63;
   end record;
   for Usage_V1'Size use 128;
   for Usage_V1'Alignment use 8;
   type Optional_Usage_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Usage_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Usage_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Usage_V1'Size use 192;
   for Optional_Usage_V1'Alignment use 8;
   type Question_Source_V1 is record
      Origin : Interfaces.Unsigned_32 := 0;
      Answered_By : Byte_String_V1 := (others => <>);
   end record with Convention => C;
   for Question_Source_V1 use record
      Origin at 0 range 0 .. 31;
      Answered_By at 8 range 0 .. 127;
   end record;
   for Question_Source_V1'Size use 192;
   for Question_Source_V1'Alignment use 8;
   type Question_Sources_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Question_Sources_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Question_Sources_V1'Size use 128;
   for Question_Sources_V1'Alignment use 8;
   type Observation_Identity_V1_Data (Arm : Positive := 1) is record
      case Arm is
         when 1 => Observation_Id : Byte_String_V1 := (others => <>);
         when 2 => Failure_Id : Byte_String_V1 := (others => <>);
         when others => null;
      end case;
   end record with Convention => C, Unchecked_Union;
   for Observation_Identity_V1_Data use record
      Observation_Id at 0 range 0 .. 127;
      Failure_Id at 0 range 0 .. 127;
   end record;
   for Observation_Identity_V1_Data'Size use 128;
   for Observation_Identity_V1_Data'Alignment use 8;
   type Observation_Identity_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Data : Observation_Identity_V1_Data := (others => <>);
   end record with Convention => C;
   for Observation_Identity_V1 use record
      Kind at 0 range 0 .. 31;
      Data at 8 range 0 .. 127;
   end record;
   for Observation_Identity_V1'Size use 192;
   for Observation_Identity_V1'Alignment use 8;
   type Observation_Identities_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Observation_Identities_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Observation_Identities_V1'Size use 128;
   for Observation_Identities_V1'Alignment use 8;
   type Profile_Warning_V1 is record
      Tuned_For : Byte_String_V1 := (others => <>);
      Running : Byte_String_V1 := (others => <>);
   end record with Convention => C;
   for Profile_Warning_V1 use record
      Tuned_For at 0 range 0 .. 127;
      Running at 16 range 0 .. 127;
   end record;
   for Profile_Warning_V1'Size use 256;
   for Profile_Warning_V1'Alignment use 8;
   type Optional_Profile_Warning_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Profile_Warning_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Profile_Warning_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 255;
   end record;
   for Optional_Profile_Warning_V1'Size use 320;
   for Optional_Profile_Warning_V1'Alignment use 8;
   type Batch_V1 is record
      Kind : Interfaces.Unsigned_32 := 0;
      Records : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Batch_V1 use record
      Kind at 0 range 0 .. 31;
      Records at 8 range 0 .. 63;
   end record;
   for Batch_V1'Size use 128;
   for Batch_V1'Alignment use 8;
   type Optional_Batch_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Batch_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Batch_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Batch_V1'Size use 192;
   for Optional_Batch_V1'Alignment use 8;
   type Batch_Warning_V1 is record
      Tuned_For : Batch_V1 := (others => <>);
      Running : Batch_V1 := (others => <>);
   end record with Convention => C;
   for Batch_Warning_V1 use record
      Tuned_For at 0 range 0 .. 127;
      Running at 16 range 0 .. 127;
   end record;
   for Batch_Warning_V1'Size use 256;
   for Batch_Warning_V1'Alignment use 8;
   type Optional_Batch_Warning_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Batch_Warning_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Batch_Warning_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 255;
   end record;
   for Optional_Batch_Warning_V1'Size use 320;
   for Optional_Batch_Warning_V1'Alignment use 8;
   type Attempt_V1 is record
      Ordinal : Interfaces.Unsigned_64 := 0;
      Request_Sha256 : Byte_String_V1 := (others => <>);
      Wall_Ms : Interfaces.Unsigned_64 := 0;
      Outcome : Interfaces.Unsigned_32 := 0;
      Sdk_Request_Id : Byte_String_V1 := (others => <>);
      Status : Optional_U16_V1 := (others => <>);
      Server_Ms : Optional_U64_V1 := (others => <>);
      Request_Id : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Attempt_V1 use record
      Ordinal at 0 range 0 .. 63;
      Request_Sha256 at 8 range 0 .. 127;
      Wall_Ms at 24 range 0 .. 63;
      Outcome at 32 range 0 .. 31;
      Sdk_Request_Id at 40 range 0 .. 127;
      Status at 56 range 0 .. 63;
      Server_Ms at 64 range 0 .. 127;
      Request_Id at 80 range 0 .. 191;
   end record;
   for Attempt_V1'Size use 832;
   for Attempt_V1'Alignment use 8;
   type Attempts_V1 is record
      Data : System.Address := System.Null_Address;
      Len : Interfaces.C.size_t := 0;
   end record with Convention => C;
   for Attempts_V1 use record
      Data at 0 range 0 .. 63;
      Len at 8 range 0 .. 63;
   end record;
   for Attempts_V1'Size use 128;
   for Attempts_V1'Alignment use 8;
   type Optional_Attempts_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Attempts_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Attempts_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 127;
   end record;
   for Optional_Attempts_V1'Size use 192;
   for Optional_Attempts_V1'Alignment use 8;
   type Meta_V1 is record
      Tool : Byte_String_V1 := (others => <>);
      Question_Sha256 : Optional_String_V1 := (others => <>);
      Questions_Sha256 : Optional_String_V1 := (others => <>);
      Url : Byte_String_V1 := (others => <>);
      Model : Byte_String_V1 := (others => <>);
      Usage : Optional_Usage_V1 := (others => <>);
      Requests_Sent : Interfaces.Unsigned_64 := 0;
      Cached : Interfaces.C.int := 0;
      Requests : Strings_V1 := (others => <>);
      Failed_Questions : Interfaces.C.size_t := 0;
      Profile_Warning : Optional_Profile_Warning_V1 := (others => <>);
      Batch_Setting : Optional_Batch_V1 := (others => <>);
      Batch_Warning : Optional_Batch_Warning_V1 := (others => <>);
      Context_Sha256 : Optional_String_V1 := (others => <>);
      Attempts : Optional_Attempts_V1 := (others => <>);
      Origin : Optional_Discriminator_V1 := (others => <>);
      Question_Sources : Question_Sources_V1 := (others => <>);
      Observations : Observation_Identities_V1 := (others => <>);
      Answered_By : Optional_String_V1 := (others => <>);
   end record with Convention => C;
   for Meta_V1 use record
      Tool at 0 range 0 .. 127;
      Question_Sha256 at 16 range 0 .. 191;
      Questions_Sha256 at 40 range 0 .. 191;
      Url at 64 range 0 .. 127;
      Model at 80 range 0 .. 127;
      Usage at 96 range 0 .. 191;
      Requests_Sent at 120 range 0 .. 63;
      Cached at 128 range 0 .. 31;
      Requests at 136 range 0 .. 127;
      Failed_Questions at 152 range 0 .. 63;
      Profile_Warning at 160 range 0 .. 319;
      Batch_Setting at 200 range 0 .. 191;
      Batch_Warning at 224 range 0 .. 319;
      Context_Sha256 at 264 range 0 .. 191;
      Attempts at 288 range 0 .. 191;
      Origin at 312 range 0 .. 63;
      Question_Sources at 320 range 0 .. 127;
      Observations at 336 range 0 .. 127;
      Answered_By at 352 range 0 .. 191;
   end record;
   for Meta_V1'Size use 3008;
   for Meta_V1'Alignment use 8;
   type Optional_Meta_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Meta_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Meta_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 3007;
   end record;
   for Optional_Meta_V1'Size use 3072;
   for Optional_Meta_V1'Alignment use 8;
   type Facts_V1 is record
      Call_Id : Byte_String_V1 := (others => <>);
      Cache_Answers : Interfaces.Unsigned_64 := 0;
      Estimated_Cost_Usd : Optional_String_V1 := (others => <>);
      Input_Tokens : Optional_U64_V1 := (others => <>);
      Model : Optional_String_V1 := (others => <>);
      Output_Tokens : Optional_U64_V1 := (others => <>);
      Records : Interfaces.Unsigned_64 := 0;
      Requests_Sent : Interfaces.Unsigned_64 := 0;
      Seconds : Interfaces.C.double := 0.0;
      Command_Ms : Optional_U64_V1 := (others => <>);
   end record with Convention => C;
   for Facts_V1 use record
      Call_Id at 0 range 0 .. 127;
      Cache_Answers at 16 range 0 .. 63;
      Estimated_Cost_Usd at 24 range 0 .. 191;
      Input_Tokens at 48 range 0 .. 127;
      Model at 64 range 0 .. 191;
      Output_Tokens at 88 range 0 .. 127;
      Records at 104 range 0 .. 63;
      Requests_Sent at 112 range 0 .. 63;
      Seconds at 120 range 0 .. 63;
      Command_Ms at 128 range 0 .. 127;
   end record;
   for Facts_V1'Size use 1152;
   for Facts_V1'Alignment use 8;
   type Optional_Facts_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Facts_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Facts_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 1151;
   end record;
   for Optional_Facts_V1'Size use 1216;
   for Optional_Facts_V1'Alignment use 8;
   type Stopped_V1 is record
      At_Index : Optional_Size_V1 := (others => <>);
      Cause : Interfaces.Unsigned_32 := 0;
      Status : Optional_U16_V1 := (others => <>);
      Retryable : Interfaces.C.int := 0;
   end record with Convention => C;
   for Stopped_V1 use record
      At_Index at 0 range 0 .. 127;
      Cause at 16 range 0 .. 31;
      Status at 20 range 0 .. 63;
      Retryable at 28 range 0 .. 31;
   end record;
   for Stopped_V1'Size use 256;
   for Stopped_V1'Alignment use 8;
   type Optional_Stopped_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Stopped_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Stopped_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 255;
   end record;
   for Optional_Stopped_V1'Size use 320;
   for Optional_Stopped_V1'Alignment use 8;
   type Error_V1 is record
      Code : Interfaces.C.int := 0;
      Message : Byte_String_V1 := (others => <>);
      Retryable : Interfaces.C.int := 0;
      Stopped : Optional_Stopped_V1 := (others => <>);
   end record with Convention => C;
   for Error_V1 use record
      Code at 0 range 0 .. 31;
      Message at 8 range 0 .. 127;
      Retryable at 24 range 0 .. 31;
      Stopped at 32 range 0 .. 319;
   end record;
   for Error_V1'Size use 576;
   for Error_V1'Alignment use 8;
   type Optional_Error_V1 is record
      Present : Interfaces.C.int := 0;
      Value : Error_V1 := (others => <>);
   end record with Convention => C;
   for Optional_Error_V1 use record
      Present at 0 range 0 .. 31;
      Value at 8 range 0 .. 575;
   end record;
   for Optional_Error_V1'Size use 640;
   for Optional_Error_V1'Alignment use 8;
end Thinkthen_C_Metadata;
